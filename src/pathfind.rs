use crate::common::{ 
        Action,
		AgentInfo,
		Coords,
        Command,
		Direction,
		CellType,
		Map,
};
use enum_iterator::IntoEnumIterator;
use crate::think::hive_cell;
use crate::utils::{coords_to_dir};
use pathfinding::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// The a-star algorithm requires the use of a tuple struct. Pos is used for
/// this purpose, as a tuple struct version of Coords.
pub struct Pos(pub i32, pub i32);

impl Pos {
    // Returns vec of empty neighbour coords
    pub fn neighbours(&self, map: &Map) -> Vec<(Pos, usize)> {
        let mut neighbours: Vec<(Pos, usize)> = Vec::new();
        let coord: &Coords = &Coords { row: self.0 as usize, col: self.1 as usize };
		for direction in Direction::into_enum_iter() {
			let adjacent = coord.adjacent_coord(&direction);
			match adjacent {
				Some(v) => {
					let cell = map.cells.get(v.row, v.col);
					match cell {
						Some(c) => {
							if c.celltype == CellType::EMPTY {
                                let pos = Pos(v.row as i32, v.col as i32);
                                neighbours.push((pos, 1));
							}
						}
						None => { continue; },
					}					
				}
				None => { continue; },
			}
		}
        neighbours
      }

    pub fn neighbours_collect(&self, map: &Map) -> Vec<(Pos, usize)> {
        let mut neighbours: Vec<(Pos, usize)> = Vec::new();
        let coord: &Coords = &Coords { row: self.0 as usize, col: self.1 as usize };
		for direction in Direction::into_enum_iter() {
			let adjacent = coord.adjacent_coord(&direction);
			match adjacent {
				Some(v) => {
					let cell = map.cells.get(v.row, v.col);
					match cell {
						Some(c) => {
							if c.celltype == CellType::EMPTY {
                                let pos = Pos(v.row as i32, v.col as i32);
                                neighbours.push((pos, 1));
							} else if c.celltype == CellType::WALL {
                                let pos = Pos(v.row as i32, v.col as i32);
                                neighbours.push((pos, 2));
                            } else if c.celltype.is_hive() {
                                let pos = Pos(v.row as i32, v.col as i32);
                                neighbours.push((pos, 1));
                            }
						}
						None => { continue; },
					}					
				}
				None => { continue; },
			}
		}
        neighbours
      }
	pub fn distance(&self, other: &Pos) -> usize {
		((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
	}
}

pub fn pathfind(info: &AgentInfo, map: &Map, dest_coord: &Coords) -> Option<Command> {
	//println!("\x1b[96mpathfind(): current bee coords: {:?} | {:?}\x1b[0m", info.row, info.col);
	//println!("\x1b[96mpathfind(): dest coords: {:?} \x1b[0m", dest_coord);
    let destination = Pos(dest_coord.row as i32, dest_coord.col as i32);
	let path = astar(&Pos(info.row as i32, info.col as i32), |p| p.neighbours(map), |p| p.distance(&destination), |p| *p == destination);
	match path {
		Some(v) => {
			//println!("\x1b[96mpathfind(): FOUND A PATH\x1b[0m");
			if let Some(pos) = v.0.get(1) {
				let next = Coords { row: pos.0 as usize, col: pos.1 as usize };
				let current = Coords { row: info.row as usize, col: info.col as usize };
				return Some(Command {
					action: Action::MOVE,
					direction: coords_to_dir(current, next),
				})
			} else {
				return None
			}
		},
		None => None,
	}
}

pub fn pathfind_collect(info: &AgentInfo, map: &Map, dest_coord: &Coords) -> Option<Command> {
  let destination = Pos(dest_coord.row as i32, dest_coord.col as i32);
	let path = astar(&Pos(info.row as i32, info.col as i32), |p| p.neighbours_collect(map), |p| p.distance(&destination), |p| *p == destination);
	match path {
		Some(v) => {
			if let Some(pos) = v.0.get(1) {
				let next = Coords { row: pos.0 as usize, col: pos.1 as usize };
				let current = Coords { row: info.row as usize, col: info.col as usize };
                let next_cell = map.cells.get(next.row, next.col).unwrap();
                if next_cell.celltype.eq(&CellType::WALL) {
				    return Some(Command {
				    	action: Action::GUARD,
				    	direction: coords_to_dir(current, next),
				    })
                } else if next_cell.celltype.eq(&hive_cell(info.player)) {
                    return None
                } else {
				    return Some(Command {
				    	action: Action::MOVE,
				    	direction: coords_to_dir(current, next),
				    })
                }
			} else {
				return None
			}
		},
		None => None,
	}
}
