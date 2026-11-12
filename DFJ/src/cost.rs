use serde::Deserialize;
use std::{error::Error, fs::File};

#[derive(Debug, Deserialize)]
struct Cost {
	origin: usize,
	destination: usize,
	distance: usize,
	time: usize,
}

#[derive(Debug)]
pub struct CostMatrix {
	ids: Vec<usize>,
	times: Vec<Vec<usize>>,
}

impl CostMatrix {
	// Create a cost matrix from the given CSV file path.
	pub fn from(filename: &str) -> Self {
		let costs = Self::parse_costs(filename).expect("Failed to parse CSV");
		let ids = Self::get_origins(&costs);
		let times = Self::time_matrix(&ids, &costs);
		Self { ids, times }
	}

	// Access the array of IDs.
	pub fn get_ids(&self) -> &[usize] {
		&self.ids
	}

	// Access the matrix of times.
	pub fn get_times(&self) -> &[Vec<usize>] {
		&self.times
	}

	// Return the total time cost of the given tour.
	pub fn time_cost(&self, tour: &[usize]) -> usize {
		let next = tour.iter().skip(1).chain(tour.first());
		tour.iter().zip(next).map(|(&from, &to)| self.times[from][to]).sum()
	}

	// Return the parsed data from the CSV of costs.
	fn parse_costs(filename: &str) -> Result<Vec<Cost>, Box<dyn Error>> {
		let file = File::open(filename)?;
		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false).from_reader(file);
		let results = reader.deserialize().collect::<Result<_, _>>()?;
		Ok(results)
	}

	// Return a sorted list of locations from the cost data.
	fn get_origins(costs: &[Cost]) -> Vec<usize> {
		let mut origins = costs.iter().map(|c| c.origin).collect::<Vec<_>>();
		origins.sort_unstable();
		origins.dedup();
		origins
	}

	// Construct a matrix of time costs for each pair of locations.
	fn time_matrix(ids: &[usize], costs: &[Cost]) -> Vec<Vec<usize>> {
		let mut matrix = vec![vec![0; ids.len()]; ids.len()];
		for cost in costs {
			let oi = ids.binary_search(&cost.origin).unwrap();
			let di = ids.binary_search(&cost.destination).unwrap();
			matrix[oi][di] = cost.time;
		}
		matrix
	}
}
