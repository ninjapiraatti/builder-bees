use crate::common::{ 
		AgentInfo,
		Coords,
		Direction,
		CellType,
		Map,
};
use std::collections::{
	HashMap,
	HashSet, 
	BTreeSet,
};
use enum_iterator::IntoEnumIterator;
use crate::utils::{coords_to_dir};

pub fn pathfind(info: &AgentInfo, map: &Map, destination: &Coords) -> Option<Direction> {
	//println!("\x1b[96m\n\n\nHenlo! Starting the astar.\n\x1b[0m");

	let mut visited = HashSet::new();
	let mut frontier = BTreeSet::new();
	let mut came_from = HashMap::<Coords, Coords>::new();
	//let mut came_from = HashMap::new();
	let mut coords = Coords {
		row: info.row as usize,
		col: info.col as usize,
	};
	//let heuristic = heuristic_cost_estimate(&coords, &destination);
	frontier.insert(coords.clone());
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
					//let value = heuristic_cost_estimate(&v, &destination);
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

/*
pub fn pathfind(info: &AgentInfo, destination: &Coords) -> Vec<Coords> {
	println!("\x1b[96m\n\n\nHenlo! Starting the astar.\n\x1b[0m");
	let mut path = Vec::new();
	let mut coords = Coords {
		row: info.row as usize,
		col: info.col as usize,
	};
	//println!("\x1b[96mCoords: {:?} \x1b[0m", coords);
	let mut open_set = BTreeSet::new();
	//let mut open_set: Vec<Coords> = vec![coords.clone()];
	//let mut closed_set = HashSet::new();
	let mut g_score = HashMap::new(); 
	let mut f_score = HashMap::new();
	//let mut came_from = HashMap::new();
	
	g_score.insert(coords, 0);
	let heuristic = heuristic_cost_estimate(&coords, &destination);
	f_score.insert(coords, heuristic);
	println!("\x1b[96mFscore with coords {:?} \x1b[0m", f_score[&coords]);
	open_set.insert(f_score[&coords]);
	println!("\x1b[96mHeuristic: {:?} \x1b[0m", heuristic);

	while !open_set.is_empty() {
		let current = open_set.get(&0).unwrap_or_else(|| panic!("No current"));
		if current == &0 {
			break;
		}
		closed_set.insert(current);
		//println!("\x1b[96mCurrent Cell: {:?} \x1b[0m", current);
		println!("\x1b[96mOpen set not empty \x1b[0m");
		/*
		for direction in Direction::into_enum_iter() {
			let adjacent = coords.adjacent_coord(&direction);
			match adjacent {
				Some(v) => {
					let y = v.row;
					let x = v.col;
					let value = heuristic_cost_estimate(&v, &destination);
					let cell_type = info.cell_type(&v);
					if cell_type.is_wall() || closed_set.contains(&value) {
						continue;
					}
					let tentative_g_score = g_score[&coords] + 1;
					if !open_set.contains(&value) || tentative_g_score < g_score[&v] {
						came_from.insert(v, coords);
						g_score.insert(v, tentative_g_score);
						f_score.insert(v, g_score[&v] + heuristic_cost_estimate(&v, &destination));
					}
				}
				None => continue,
			}
		}*/
	}
	println!("\x1b[96mPath: {:?} \x1b[0m", path);
	path
}
*/
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