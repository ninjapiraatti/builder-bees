use crate::common::{ 
		AgentInfo,
		Coords,
		Direction,
		CellType,
		Cell,
		Map,
};
use std::collections::{
	HashMap,
	HashSet, 
	BTreeMap,
};
use enum_iterator::IntoEnumIterator;
use crate::utils::{coords_to_dir};
/* 
pub fn pathfind(info: &AgentInfo, map: &Map, destination: &Coords) -> Option<Direction> {
	//println!("\x1b[96m\n\n\nHenlo! Starting the astar.\n\x1b[0m");

	let mut visited = HashSet::new();
	let mut frontier = BTreeSet::new();
	let mut came_from = HashMap::<Coords, Coords>::new();
	let mut frontier_score = HashMap::<Coords, i32>::new();
	//let mut came_from = HashMap::new();
	let mut coords = Coords {
		row: info.row as usize,
		col: info.col as usize,
	};
	let heuristic = heuristic_cost_estimate(&coords, &destination);
	frontier_score.insert(coords, heuristic);
	frontier.insert(coords);
	while !frontier.is_empty() {
		let current = frontier.iter().next().unwrap().clone();
		frontier.remove(&current);
		if current == *destination {
			let mut path = Vec::new();
			let mut current = current;
			while current != coords {
			//while current != destination {
				path.push(came_from.get(&current).unwrap().clone());
				current = came_from.get(&current).unwrap().clone();
			}
			println!("\x1b[96mBee: {:?}x1b[0m", info.bee);
			println!("\x1b[96mPath: {:?}\n\x1b[0m", path);
			let coords_to_util = path.pop().unwrap();
			let current = path.pop().unwrap();
			//println!("\x1b[96mRETURNS COORD: {:?}\n\x1b[0m", coords_to_util);
			println!("\x1b[96mCoords: {:?} | coords_to_util: {:?}\n\x1b[0m", coords, current);
			return Some(coords_to_dir(coords, current));
		}
		visited.insert(current);
		for direction in Direction::into_enum_iter() {
			let adjacent = current.adjacent_coord(&direction);
			match adjacent {
				Some(v) => {
					let value = heuristic_cost_estimate(&v, &destination);
					if visited.contains(&v) {
						//println!("\x1b[96mVisited: {:?}\x1b[0m", visited);
						continue;
					}
					//println!("\x1b[96mCoords: {:?} | coords_to_util: {:?}\n\x1b[0m", current, adjacent);
					//println!("\x1b[96mGot a coordinate: {:?}\x1b[0m", v);
					//println!("\x1b[96mMap at that coordinate: {:?}\x1b[0m", map.cells.get(v.row, v.col).unwrap().celltype);
					let cell = map.cells.get(v.row, v.col);
					match cell {
						Some(c) => {
							if c.celltype == CellType::EMPTY {
								//println!("\x1b[96mInserting coord.\x1b[0m");
								frontier.insert(v.clone());
								//println!("\x1b[96mFrontier: {:?}\x1b[0m", frontier.len());
								came_from.insert(v.clone(), current.clone());
								//println!("\x1b[96mCame from: {:?}\x1b[0m", came_from);
							}
						}
						None => continue,
					}
				},
				None => continue,
			}
		}
	}
	return None;
}

*/

pub fn pathfind(info: &AgentInfo, map: &Map, destination: &Coords) -> Option<Direction> {
	//println!("\x1b[96m\n\n\nHenlo! Starting the astar.\n\x1b[0m");
	println!("\x1b[96mStarting pathfind. \x1b[0m");
	let mut coords = Coords {
		row: info.row as usize,
		col: info.col as usize,
	};
	//println!("\x1b[96mCoords: {:?} \x1b[0m", coords);
	let mut open_set = BTreeMap::new(); // Frontier
	//let mut open_set: Vec<Coords> = vec![coords.clone()];
	//let mut open_set = HashMap::<Coords, i32>::new(); // Frontier
	let mut g_score = HashMap::new(); // Cost so far
	let mut f_score = HashMap::new();
	let mut came_from = HashMap::new(); // Came from
	let mut debug_count = 0;
	let mut debug_count2 = 0;
	let mut debug_coords = Coords {
		row: 0,
		col: 0,
	};
	let mut debug_cell = Cell {
		celltype: CellType::EMPTY,
		heat: 0.0,
		is_destination: false,
		is_target: false,
	};

	let heuristic = heuristic_cost_estimate(&coords, &destination);
	g_score.insert(coords, 0);
	f_score.insert(coords, heuristic);
	//println!("\x1b[96mFscore {:?} with coords {:?} \x1b[0m", f_score[&coords], coords);
	open_set.insert(heuristic, coords);
	//println!("\x1b[96mHeuristic: {:?} \x1b[0m", heuristic);
	while !open_set.is_empty() {
		//let current = open_set.get(&0).unwrap_or_else(|| panic!("No current"));
		//let current = open_set.keys().next().unwrap().clone(); // This def aint right
		//let current = open_set.remove(&open_set.keys().next_back().unwrap().clone()).unwrap();
		let current = open_set.remove(&open_set.keys().next().unwrap().clone()).unwrap();
		//let current = open_set.remove(&open_set.keys().next().unwrap().clone()).unwrap();
		debug_count = open_set.len();
		if current == *destination {
			println!("\x1b[96m\nACTUALLY FOUND A PATH!\n\x1b[0m");
			//println!("\x1b[96mCame from: {:?} \x1b[0m", came_from.len());
			println!("\x1b[96mDebug number: {:?} \x1b[0m", debug_count);
			/*
			let mut path = Vec::new();
			let mut current = current;
			while current != coords {
				path.push(came_from.get(&current).unwrap().clone());
				current = came_from.get(&current).unwrap().clone();
			}
			*/
			return Some(coords_to_dir(coords, current));
		}
		for direction in Direction::into_enum_iter() {
			let adjacent = current.adjacent_coord(&direction);
			match adjacent {
				Some(v) => {
					let cell = map.cells.get(v.row, v.col);
					debug_coords = v.clone();
					match cell {
						Some(c) => {
							let debug_cell = c;
							if c.celltype == CellType::EMPTY {
								let heur_value = heuristic_cost_estimate(&v, &destination);
								let tentative_g_score = g_score.get(&current).unwrap_or(&1000000) + heur_value;
								if !open_set.contains_key(&heur_value) || tentative_g_score < *g_score.get(&v).unwrap_or(&1000000) {
									came_from.insert(current, v);
									g_score.insert(v, tentative_g_score);
									open_set.insert(heur_value, v);
								}
							}
						}
						None => {
							//println!("\x1b[96mNO CELL! Debug number: {:?} \x1b[0m", debug_count);
							continue;
						},
					}					
				}
				None => {
					//println!("\x1b[96mNO ADJACENT! Debug number: {:?} \x1b[0m", debug_count);
					continue;
				},
			}
		}
		//println!("\x1b[96mEnd of Direction iteration. Debug number: {:?} \x1b[0m", debug_count);
		//println!("\x1b[96mCurrent: {:?} \x1b[0m", current);
		//println!("\x1b[96mDestination: {:?} \x1b[0m", destination);
		//println!("\x1b[96mOpen set length: {:?} \x1b[0m", debug_count);
	}
	//println!("\x1b[96mBee: {:?} \x1b[0m", info.bee);
	//println!("\x1b[96mCame from: {:?} \x1b[0m", came_from);
	println!("\x1b[96mDebug number: {:?} \x1b[0m", debug_count);
	println!("\x1b[96mDebug number 2: {:?} \x1b[0m", debug_count2);
	println!("\x1b[96mDebug thing: {:?} \x1b[0m", debug_coords);
	println!("\x1b[96mDebug cell: {:?} \x1b[0m", debug_cell);
	println!("\x1b[96mOpen Set is empty.\x1b[0m");
	None
}

fn heuristic_cost_estimate(from: &Coords, to: &Coords) -> i32 {
	//println!("\x1b[96mFromCoords: {:?} \x1b[0m", from);
	//println!("\x1b[96mToCoords: {:?} \x1b[0m", to);
	let dx = (to.row as i32 - from.row as i32) as f32;
	let dy = (to.col as i32 - from.col as i32) as f32;
	//((dx * dx) + (dy * dy)).sqrt() as i32
	//println!("\x1b[96mDx | Dy: {:?} {:?} \x1b[0m", dx, dy);
	let cost_float = (dx * dx + dy * dy).sqrt();
	//println!("\x1b[96mCostFloat: {:?} \x1b[0m", cost_float);
	let cost_int = (cost_float * 10000.0).round() as i32;
	//println!("\x1b[96mCostInt: {:?} \x1b[0m", cost_int);
	cost_int
}