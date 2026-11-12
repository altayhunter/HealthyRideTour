mod cost;
mod solver;
use crate::cost::CostMatrix;
use crate::solver::find_optimal_tour;

fn main() {
	let costs = CostMatrix::from("../data/costs.csv");
	if let Err(err) = find_optimal_tour(costs.get_times(), costs.get_ids()) {
		eprintln!("Optimization failed: {}", err);
		std::process::exit(1);
	}
}
