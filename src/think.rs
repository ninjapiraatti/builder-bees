// This file contains functions used by the agent to choose an appropriate action.

use crate::common::{ AgentInfo, Cell, Coords, Direction, VIEW_DISTANCE };
use array2d::Array2D;
use enum_iterator::IntoEnumIterator;

/// Checks if surrounding cells have a cell of cell_type.
/// If found, returns the direction of that cell wrapped in Option.
pub fn find_neighbour(info: &AgentInfo, cell_type: &Cell) -> Option<Direction> {
	let coords = Coords {
		row: VIEW_DISTANCE,
		col: VIEW_DISTANCE,
	};
	for direction in Direction::into_enum_iter() {
		let adjacent = coords.adjacent_coord(&direction);
		match adjacent {
			Some(v) if info.cell_type(&v).eq(cell_type) => return Some(direction),
			Some(_) => continue, //TODO: Figure out if this is ok
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
				println!("{:?} | {:?}", max_direction, heat);
			},
			None => continue,
		}
	}
	Some(max_direction)
}
