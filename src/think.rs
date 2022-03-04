// This file contains functions used by the agent to choose an appropriate action.

use crate::common::{ AgentInfo, Cell, Coords, Direction, VIEW_DISTANCE };
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
