// This file contains functions used by the agent to choose an appropriate action.

use crate::common::{
	AgentInfo,
	Cell,
	CellType,
	Coords,
	Direction,
	NUM_ROWS,
	NUM_COLS,
	VIEW_DISTANCE
};
use crate::bee::{ Bee, Role };
use array2d::Array2D;
use enum_iterator::IntoEnumIterator;

/// Returns the hive cell type given a player number.
pub fn hive_cell(player: i32) -> CellType {
		if player == 0 {
				CellType::HIVE_0
		} else {
				CellType::HIVE_1
		}
}

/// Returns the hive coords given a player number.
pub fn hive_coords(player: i32) -> Coords {
		if player == 0 {
				Coords { row: 12, col: 1 }
		} else {
				Coords { row: 12, col: NUM_COLS - 2 }
		}
}

pub fn defender_coords(player: i32) -> Coords {
		if player == 0 {
				Coords { row: 11, col: 2 }
		} else {
				Coords { row: 11, col: NUM_COLS - 3 }
		}
}

/// Checks if it is possible for a bee to move in a certain direction.
pub fn can_move_in_direction(info: &AgentInfo, direction: &Direction) -> bool {
		let position = Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE };
		let adjacent = position.adjacent_coord(direction).unwrap();
		if CellType::EMPTY.eq(&info.cells.get(adjacent.row, adjacent.col).unwrap()) {
				return true
		}
		return false
}

/// Gets the direction that can be used to move toward destination.
pub fn get_direction(destination: &Coords, position: &Coords) -> Option<Direction> {
		match destination {
				&Coords { row, col } if row < position.row && col == position.col => Some(Direction::N),
				&Coords { row, col } if row < position.row && col > position.col => Some(Direction::NE),
				&Coords { row, col } if row == position.row && col > position.col => Some(Direction::E),
				&Coords { row, col } if row > position.row && col > position.col => Some(Direction::SE),
				&Coords { row, col } if row > position.row && col == position.col => Some(Direction::S),
				&Coords { row, col } if row > position.row && col < position.col => Some(Direction::SW),
				&Coords { row, col } if row == position.row && col < position.col => Some(Direction::W),
				&Coords { row, col } if row < position.row && col < position.col => Some(Direction::NW),
				_ => None
		}
}

/// Checks if surrounding cells have a cell of cell_type.
/// If found, returns the direction of that cell wrapped in Option.
pub fn find_neighbour(info: &AgentInfo, cell_type: &CellType) -> Option<Direction> {
	let coords = Coords {
		row: VIEW_DISTANCE,
		col: VIEW_DISTANCE,
	};
	for direction in Direction::into_enum_iter() {
		let adjacent = coords.adjacent_coord(&direction);
		match adjacent {
			Some(v) if info.cell_type(&v).eq(cell_type) => return Some(direction),
			Some(_) => continue,
			None => continue,
		}
	}
	None
}

pub fn find_available_adjacent(target: Coords, map: &Array2D<Cell>) -> Option<Coords> {
	if target.col >= NUM_COLS || target.row >= NUM_ROWS {
		return None;
	}
	for direction in Direction::into_enum_iter() {
		let adjacent = target.adjacent_coord(&direction);
		match adjacent {
			Some(v) if map.get(v.row, v.col).unwrap().celltype.eq(&CellType::EMPTY) => return Some(v),
			Some(_) => continue,
			None => continue,
		}
	}
	None
}

pub fn find_heat(info: &AgentInfo, heatmap: &Array2D<f32>) -> Option<Direction> {
	let coords = Coords {
		row: info.row as usize,
		col: info.col as usize,
	};
	let mut min_heat = 100.0;
	let mut max_direction = Direction::N;
	for direction in Direction::into_enum_iter() {
		let adjacent = coords.adjacent_coord(&direction);
		match adjacent {
			Some(v) => {
				let y = v.row;
				let x = v.col;
				let heat = heatmap.get(y, x).unwrap_or(&100.0);
				if heat < &min_heat {
					min_heat = *heat;
					max_direction = direction;
				}
			},
			None => continue,
		}
	}
	Some(max_direction)
}

/// Finds wall building target for builder bee.
pub fn find_target(info: &AgentInfo, bee: &Bee, heatmap: &Array2D<f32>, map: &Array2D<Cell>, targets: &Vec<Coords>) -> Option<Coords> {
	if bee.role.as_ref().unwrap().eq(&Role::Collect) {
		let flower_coords = find_flower_in_map(map, &bee.position);
		if flower_coords.is_some() {
			let flower_target = find_available_adjacent(flower_coords.unwrap(), map);
			println!("\x1b[96mfind_target(): Returning {:?}\x1b[0m", flower_target);
			return flower_target;
		}
		return None
	}
	let mut min_heat = 100.0;
	let mut min_row = 100;
	let mut min_col = 100;
	for row in 0..NUM_ROWS {
		'cols: for col in 0..NUM_COLS {
			let cell = map.get(row, col).unwrap();
			if cell.celltype.eq(&CellType::WALL) || cell.celltype.is_hive() { continue; };
			let current = Coords { row: row, col: col };
			for target in targets {
				if target.row == current.row && target.col == current.col {
				//if target.eq(&current) { 
					//println!("\x1b[96mCurrent coords: {:?} | Potential target: {:?} \x1b[0m", current, target);
					//println!("\x1b[96mThey were the same. \x1b[0m");
					continue 'cols;
				};
			}
			let heat = heatmap.get(row, col).unwrap_or(&100.0);
			if heat < &min_heat {
				//println!("\x1b[93mFound better. \x1b[0m");
				min_heat = *heat;
				min_row = row;
				min_col = col;
			}
		}
	}
	if min_row == 100 || min_col == 100 { return None };
	//println!("\x1b[93mReturning: {:?} \x1b[0m", Coords { row: min_row, col: min_col });
	Some(Coords { row: min_row, col: min_col })
}

/// If a flower is in view, return its coordinates.
pub fn find_flower_in_view(info: &AgentInfo) -> Option<Coords> {
	for row in 0..7 {
		for col in 0..7 {
			let cell = info.cells.get(row, col).unwrap();
			if CellType::FLOWER.eq(cell) {
				let coords = Coords {
					row: row,
					col: col,
				};
				return Some(coords)
			}
		}
	}
	None
}

/// Find nearest flower in map.
pub fn find_flower_in_map(map: &Array2D<Cell>, bee_coords: &Coords) -> Option<Coords> {
    let mut result = None;
    let mut result_dist = 100;
	for row in 0..NUM_ROWS {
		for col in 0..NUM_COLS {
			let cell = map.get(row, col).unwrap();
			if CellType::FLOWER.eq(&cell.celltype) {
				let coords = Coords {
					row: row,
					col: col,
				};
                let dist = bee_coords.distance(&coords);
                if dist < result_dist {
                    result = Some(coords);
                    result_dist = dist;
                }
			}
		}
	}
	return result
}
