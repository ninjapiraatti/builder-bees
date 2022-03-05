// This file contains functions used by the agent to choose an appropriate action.

use crate::common::{ AgentInfo, Cell, Coords, Direction, NUM_COLS, VIEW_DISTANCE };
use array2d::Array2D;
use enum_iterator::IntoEnumIterator;

/// Returns the hive cell type given a player number.
pub fn hive_cell(player: i32) -> Cell {
    if player == 0 {
        Cell::HIVE_0
    } else {
        Cell::HIVE_1
    }
}

/// Returns the hive coords type given a player number.
pub fn hive_coords(player: i32) -> Coords {
    if player == 0 {
        Coords { row: 12, col: 1 }
    } else {
        Coords { row: 12, col: NUM_COLS - 1 }
    }
}

//TODO: Doesn't work with hive_coords because hive_coords gives map coords
pub fn get_direction_to_destination(destination: &Coords) -> Direction {
    match destination {
        &Coords { row, col } if row < VIEW_DISTANCE && col == VIEW_DISTANCE => Direction::N,
        &Coords { row, col } if row < VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::NE,
        &Coords { row, col } if row == VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::E,
        &Coords { row, col } if row > VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::SE,
        &Coords { row, col } if row > VIEW_DISTANCE && col == VIEW_DISTANCE => Direction::S,
        &Coords { row, col } if row > VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::SW,
        &Coords { row, col } if row == VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::W,
        &Coords { row, col } if row < VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::NW,
        &_ => Direction::S, //TODO:Fix this
    }
}

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

/// If a flower is in view, return its coordinates.
pub fn find_flower_in_view(info: &AgentInfo) -> Option<Coords> {
    for row in 0..7 {
        for col in 0..7 {
            let cell = info.cells.get(row, col).unwrap();
            if Cell::FLOWER.eq(cell) {
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
