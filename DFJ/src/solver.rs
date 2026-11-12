use anyhow::{bail, Result};
use contracts::requires;
use good_lp::{
	constraint, highs, variable, variables,
	Expression, ProblemVariables, Solution, SolverModel, Variable,
};
use good_lp::solvers::highs::HighsProblem;

/// Prove whether the best-known tour cost is optimal using DFJ cuts.
#[requires(
	costs.iter().all(|row| row.len() == costs.len()),
	"Cost matrix must be square",
)]
#[requires(ids.len() == costs.len(), "Node count must match cost matrix rows")]
pub fn find_optimal_tour(costs: &[Vec<usize>], ids: &[usize]) -> Result<()> {
	let max_iterations = 500;
	let (vars, x) = make_variables(ids.len());
	let objective = make_objective(&costs, &x);
	let mut cuts = vec![];
	for iteration in 1..=max_iterations {
		let model = make_model(vars.clone(), objective.clone(), &cuts, &x);
		let solution = model.solve()?;
		let (candidate, cost) = extract_solution(&solution, &x, costs);
		let cycles = find_cycles(&candidate);
		print_result(cost, &cycles, ids, iteration);
		if cycles.len() == 1 {
			println!("This is the global optimum.");
			return Ok(());
		}
		let cutset_constraints = cycles.into_iter()
			.map(|cycle| build_cutset_constraint(&cycle, &x));
		cuts.extend(cutset_constraints);
	}
	bail!("Error: {max_iterations} iterations reached without convergence.");
}

/// Create an n x n matrix of variables.
fn make_variables(n: usize) -> (ProblemVariables, Vec<Vec<Option<Variable>>>) {
	let mut vars = variables!();
	let mut x = vec![vec![None; n]; n];
	for (i, j) in directed_pairs(n) {
		x[i][j] = Some(vars.add(variable().binary()));
	}
	(vars, x)
}

/// Create an objective by combining the given variables with their respectives
/// costs.
fn make_objective(
	costs: &[Vec<usize>],
	x: &[Vec<Option<Variable>>],
) -> Expression {
	let mut objective = Expression::default();
	for (i, j) in directed_pairs(costs.len()) {
		objective += (costs[i][j] as f64) * x[i][j].unwrap();
	}
	objective
}

/// Create an optimization model to minimize the given objective with the given
/// cuts and base constraints using the simplex algorithm.
#[requires(
	x.iter().all(|row| row.len() == x.len()),
	"X must be a square matrix",
)]
fn make_model(
	vars: ProblemVariables,
	objective: Expression,
	cuts: &[Expression],
	x: &[Vec<Option<Variable>>],
) -> HighsProblem {
	let mut model = vars.minimise(objective).using(highs);
	model = add_base_constraints(model, &x);
	for cut in cuts {
		model = model.with(constraint!(cut.clone() >= 1));
	}
	model
}

/// Add the constraints that each node must have one outbound and inbound edge.
#[requires(
	x.iter().all(|row| row.len() == x.len()),
	"X must be a square matrix",
)]
fn add_base_constraints(
	mut model: HighsProblem,
	x: &[Vec<Option<Variable>>],
) -> HighsProblem {
	let mut edges_out = vec![Expression::default(); x.len()];
	let mut edges_in = vec![Expression::default(); x.len()];
	for (i, j) in directed_pairs(x.len()) {
		if let Some(var) = x[i][j] {
			edges_out[i] += var;
			edges_in[j] += var;
		}
	}
	for edge in edges_out {
		model = model.with(constraint!(edge == 1));
	}
	for edge in edges_in {
		model = model.with(constraint!(edge == 1));
	}
	model
}

/// Returns an iterator over all directed pairs where i != j for an n x n matrix.
fn directed_pairs(n: usize) -> impl Iterator<Item = (usize, usize)> {
	let mut pairs = Vec::with_capacity(n * (n - 1));
	for i in 0..n {
		for j in 0..n {
			if i == j { continue; }
			pairs.push((i, j));
		}
	}
	pairs.into_iter()
}

/// Extract the candidate tour and total cost from a solved model.
#[requires(
	x.iter().all(|row| row.len() == x.len()),
	"X must be a square matrix",
)]
#[requires(costs.len() == x.len(), "Costs must be the same size as x")]
#[requires(
	costs.iter().all(|row| row.len() == costs.len()),
	"Costs must be a square matrix",
)]
fn extract_solution(
	solution: &impl Solution,
	x: &[Vec<Option<Variable>>],
	costs: &[Vec<usize>],
) -> (Vec<usize>, usize) {
	let n = x.len();
	let mut candidate = vec![0; n];
	let mut cost = 0;
	for (i, j) in directed_pairs(n) {
		if let Some(var) = x[i][j] {
			if solution.value(var) <= 0.5 { continue; }
			candidate[i] = j;
			cost += costs[i][j];
		}
	}
	(candidate, cost)
}

/// Follow the indexes to decompose the candidate solution into disjoint cycles.
/// Any valid candidate solution must be a permutation of [0..n-1].
#[requires(
	candidate.iter().all(|&next| next < candidate.len()),
	"Candidate must only contain valid indexes",
)]
#[ensures(
	ret.iter().all(|cycle| !cycle.is_empty()),
	"Every cycle contains at least one index",
)]
#[ensures(
	ret.iter().all(|cycle| cycle.iter().all(|&i| i < candidate.len())),
	"Every cycle only contains valid indexes",
)]
fn find_cycles(candidate: &[usize]) -> Vec<Vec<usize>> {
	let mut visited = vec![false; candidate.len()];
	let mut cycles = vec![];
	for i in 0..candidate.len() {
		if visited[i] { continue; }
		let mut cycle = vec![];
		let mut next = i;
		while !visited[next] {
			visited[next] = true;
			cycle.push(next);
			next = candidate[next];
		}
		cycles.push(cycle);
	}
	cycles
}

/// Print the final optimization results and reconstructs the optimal tour path.
fn print_result(
	cost: usize,
	cycles: &[Vec<usize>],
	ids: &[usize],
	iteration: usize,
) {
	println!(
		"Iteration {iteration}: Found {} subtours with total cost {cost}.",
		cycles.len()
	);
	for cycle in cycles {
		let path: Vec<usize> = cycle.iter().map(|&i| ids[i]).collect();
		println!(" {:?}", path);
	}
}

/// Build an outbound Subtour Elimination Constraint (SEC) expression for a given
/// subtour.
/// This enforces that the sum of the edges leaving the subtour are at least one:
/// sum_{i ∈ S, j ∉ S} x_{ij} ≥ 1
#[requires(
	cycle.iter().all(|&i| i < x.len()),
	"Cycle must only contain valid indexes",
)]
#[requires(
	x.iter().all(|row| row.len() == x.len()),
	"X must be a square matrix",
)]
fn build_cutset_constraint(
	cycle: &[usize],
	x: &[Vec<Option<Variable>>],
) -> Expression {
	let subtour = cycle_to_mask(cycle, x.len());
	let mut cut = Expression::default();
	// Add all existing outbound edges to the cut.
	for &i in cycle {
		for j in 0..x.len() {
			if subtour[j] { continue; }
			if let Some(var) = x[i][j] {
				cut += var;
			}
		}
	}
	cut
}

/// Converts a subtour cycle into a boolean membership mask of size n.
#[requires(
	cycle.iter().all(|&i| i < n),
	"Cycle must only contain indexes within n",
)]
#[ensures(ret.len() == n, "Mask length matches the given size")]
#[ensures(cycle.iter().all(|&i| ret[i]), "Mask fields match cycle indexes")]
fn cycle_to_mask(cycle: &[usize], n: usize) -> Vec<bool> {
	let mut mask = vec![false; n];
	for &i in cycle {
		mask[i] = true;
	}
	mask
}

#[cfg(test)]
mod tests {
	use super::*;
	use good_lp::{IntoAffineExpression, SolutionStatus};
	use std::collections::HashMap;

	#[test]
	fn test_directed_pairs() {
		let pairs: Vec<_> = directed_pairs(3).collect();
		assert_eq!(pairs, vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]);
	}

	#[test]
	fn test_find_cycles_with_multiple_subtours() {
		let tour = vec![3, 5, 1, 0, 4, 2];
		let cycles = find_cycles(&tour);
		assert_eq!(cycles, vec![vec![0, 3], vec![1, 5, 2], vec![4]]);
	}

	#[test]
	fn test_cycle_to_mask() {
		let tour = vec![4, 7, 1, 0, 5, 2];
		let mask = cycle_to_mask(&tour, 8);
		assert_eq!(mask, vec![true, true, true, false, true, true, false, true]);
	}

	fn create_variable_matrix(n: usize) -> Vec<Vec<Option<Variable>>> {
		let mut vars = variables!();
		let mut matrix = vec![vec![None; n]; n];
		for i in 0..n {
			for j in 0..n {
				if i != j {
					matrix[i][j] = Some(vars.add(variable().binary()));
				}
			}
		}
		matrix
	}

	#[test]
	fn test_build_cutset_constraint_with_single_node_subtour() {
		let vars = create_variable_matrix(4);
		let cycle = vec![1];
		let cut_expr = build_cutset_constraint(&cycle, &vars);
		let terms: Vec<_> = cut_expr.linear_coefficients().collect();
		assert_eq!(terms.len(), 3);
		assert_eq!(terms[0], (vars[1][3].unwrap(), 1.0));
		assert_eq!(terms[1], (vars[1][2].unwrap(), 1.0));
		assert_eq!(terms[2], (vars[1][0].unwrap(), 1.0));
	}

	struct MockSolution {
		values: HashMap<Variable, f64>,
	}

	impl Solution for MockSolution {
		fn status(&self) -> SolutionStatus {
			SolutionStatus::Optimal
		}
		fn value(&self, var: Variable) -> f64 {
			*self.values.get(&var).unwrap()
		}
	}

	#[test]
	fn test_extract_solution_with_valid_tour() {
		let mut vars = variables!();
		let v01 = vars.add(variable().binary());
		let v12 = vars.add(variable().binary());
		let v20 = vars.add(variable().binary());
		let x = vec![
			vec![None, Some(v01), None],
			vec![None, None, Some(v12)],
			vec![Some(v20), None, None],
		];
		let costs = vec![
			vec![0, 10, 0],
			vec![0, 0, 15],
			vec![20, 0, 0],
		];
		let mut values = HashMap::new();
		values.insert(v01, 1.0);
		values.insert(v12, 1.0);
		values.insert(v20, 1.0);
		let solution = MockSolution { values };
		let (candidate, cost) = extract_solution(&solution, &x, &costs);
		assert_eq!(candidate, vec![1, 2, 0]);
		assert_eq!(cost, 45);
	}
}
