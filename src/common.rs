#![allow(unused)]
#![allow(non_camel_case_types)]

use array2d::Array2D;
use enum_iterator::IntoEnumIterator;
use std::fmt::{ Debug, Formatter };
use std::fmt;
use rand::distributions::{ Distribution, Standard };
use rand::Rng;
use crate::bee::Bee;

pub const NUM_ROWS: usize = 25;
pub const NUM_COLS: usize = 30;

pub const NUM_PLAYERS: i32 = 2;
pub const NUM_BEES: i32 = 5;

pub const TURNS_BEFORE_TIMEOUT: i32 = 1000;

pub const VIEW_DISTANCE: usize = 3;
pub const VIEW_SIZE: usize = VIEW_DISTANCE * 2 + 1;

pub const MAX_AGENT_INFO_LEN: usize = (VIEW_SIZE * VIEW_SIZE + 30);
pub const MAX_COMMAND_LEN: usize = 10;
pub const NET_BUFFER_SIZE: usize = 200;

pub const STRATEGY_BUILD_WALLS: i32 = 0;
pub const STRATEGY_PICK_FLOWERS: i32 = 1;

pub type ThinkFunction = fn(&AgentInfo, &Array2D<f32>, &mut GameState) -> Command;
//pub type ThinkFunction = fn(&AgentInfo, &mut GameState) -> Command;

pub struct AgentInfo {
	pub turn: i32,
	pub player: i32,
	pub bee: i32,
	pub row: i32,
	pub col: i32,
	pub cells: Array2D<CellType>,
}

impl AgentInfo {
	pub fn new() -> Self {
		Self {
			turn: 0,
			player: 0,
			bee: 0,
			row: 0,
			col: 0,
			cells: Array2D::filled_with(CellType::EMPTY, VIEW_SIZE, VIEW_SIZE),
		}
	}

	pub fn cell_type(&self, coords: &Coords) -> &CellType {
		self.cells.get(coords.row, coords.col).unwrap()
	}
}

impl Debug for AgentInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_struct("AgentInfo")
			.field("turn", &self.turn)
			.field("player", &self.player)
			.field("bee", &self.bee)
			.field("row", &self.row)
			.field("col", &self.col)
			.finish_non_exhaustive()
	}
}

pub struct Map {
	pub cells: Array2D<Cell>,
	//pub width: i32,
	//pub height: i32,
}

impl Map {
	pub fn new() -> Self {
		Self {
			cells: Array2D::filled_with(Cell::new(), NUM_ROWS, NUM_COLS),
			//width: NUM_COLS,
			//height: NUM_ROWS,
		}
	}
}

pub struct GameState {
	pub map: Map,
	pub bees: Vec<Bee>,
	pub strategy: i32,
}

impl GameState {
	pub fn new() -> Self {
		Self {
			map: Map::new(),
			bees: vec![Bee::new(0), Bee::new(1), Bee::new(2), Bee::new(3), Bee::new(4)],
			strategy: STRATEGY_BUILD_WALLS,
		}
	}

	pub fn update(&mut self, agent_info: &AgentInfo) {
		for row in 0..VIEW_SIZE {
			//print!("\n");
			for col in 0..VIEW_SIZE {
				//print!("{:?}", self.map.cells.get(row,col).unwrap());
				let y = agent_info.row + row as i32 - VIEW_DISTANCE as i32;
				let x = agent_info.col + col as i32 - VIEW_DISTANCE as i32;
				if x >= 0 && x < NUM_COLS as i32 && y >= 0 && y < NUM_ROWS as i32 {
					//println!("{:?}", self.map.cells.get(y,x).unwrap());
					self.map.cells.set(y as usize, x as usize, Cell::from(*agent_info.cells.get(row as usize, col as usize)
							.unwrap_or_else(|| &CellType::EMPTY)));
				}
			}
		}
	}

    pub fn get_targets(&self) -> Vec<Coords> {
        let mut targets: Vec<Coords> = Vec::new();
        for bee in &self.bees {
            let target = bee.target;
            if target.is_some() { targets.push(target.unwrap().clone()) };
        }
        targets
    }
}

#[derive(Debug)]
pub struct Command {
	pub action: Action,
	pub direction: Direction,
}

impl Command {
	pub fn new() -> Self {
		Self {
			action: Action::MOVE,
			direction: Direction::N,
		}
	}
}

#[derive(Debug, IntoEnumIterator)]
pub enum Direction {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}

impl Direction {
	/// Given a direction, returns the x and y offset values needed
	/// to index the cell in that direction.
	pub fn direction_offset(direction: Direction) -> (i32, i32) {
		match direction {
			Direction::N => (-1, 0),
			Direction::NE => (-1, 1),
			Direction::E => (0, 1),
			Direction::SE => (1, 1),
			Direction::S => (1, 0),
			Direction::SW => (1, -1),
			Direction::W => (0, -1),
			Direction::NW => (-1, -1),
		}
	}
}

impl Distribution<Direction> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
		match rng.gen_range(0..=7) {
			0 => Direction::N,
			1 => Direction::NE,
			2 => Direction::E,
			3 => Direction::SE,
			4 => Direction::S,
			5 => Direction::SW,
			6 => Direction::W,
			_ => Direction::NW,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Coords {
	pub row: usize,
	pub col: usize,
}

impl Coords {
	/// Given a direction, returns the coords of the adjacent cell
	/// in that direction wrapped in Option.
	pub fn adjacent_coord(&self, direction: &Direction) -> Option<Coords> {
		match direction {
			Direction::N if self.row == 0 => None,
			Direction::N => Some(Coords { row: self.row - 1, col: self.col }),
			Direction::NE if self.row == 0 || self.col == NUM_COLS - 1 => None,
			Direction::NE => Some(Coords { row: self.row - 1, col: self.col + 1 }),
			Direction::E if self.col == NUM_COLS - 1 => None,
			Direction::E => Some(Coords { row: self.row, col: self.col + 1 }),
			Direction::SE if self.row == NUM_ROWS - 1 || self.col == NUM_COLS + 1 => None,
			Direction::SE => Some(Coords { row: self.row + 1, col: self.col + 1 }),
			Direction::S if self.row == NUM_ROWS + 1 => None,
			Direction::S => Some(Coords { row: self.row + 1, col: self.col }),
			Direction::SW if self.row == NUM_ROWS + 1 || self.col == 0 => None,
			Direction::SW => Some(Coords { row: self.row + 1, col: self.col - 1 }),
			Direction::W if self.col == 0 => None,
			Direction::W => Some(Coords { row: self.row, col: self.col - 1 }),
			Direction::NW if self.row == 0 || self.col == 0 => None,
			Direction::NW => Some(Coords { row: self.row - 1, col: self.col - 1 }),
		}
	}

    pub fn is_adjacent(&self, other: &Coords) -> bool {
		for direction in Direction::into_enum_iter() {
			let adjacent = self.adjacent_coord(&direction);
			match adjacent {
				Some(v) => { if v.eq(other) { return true } },
				None => continue,
            }
        }
        false
    }

    /// Manhattan distance
	pub fn distance(&self, other: &Coords) -> usize {
		((self.row as i32 - other.row as i32).abs() + (self.col as i32 - other.col as i32).abs()) as usize
	}
}

#[derive(Debug)]
pub enum Action {
	MOVE,
	FORAGE,
	BUILD,
	GUARD,
}

#[derive(Clone, Debug)]
pub struct Cell {
	pub celltype: CellType,
	pub heat: f32,
	pub is_destination: bool,
	pub is_target: bool,
}

impl Cell {
	pub fn new() -> Self {
		Self {
			celltype: CellType::EMPTY,
			heat: 0.0, //Placeholder
			is_destination: false,
			is_target: false,
		}
	}
}

impl From<CellType> for Cell {
	fn from(v: CellType) -> Self {
		Cell {
			celltype: v,
			heat: 0.0, //Placeholder
			is_destination: false,
			is_target: false,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellType {
	EMPTY,
	BEE_0,
	BEE_1,
	BEE_0_WITH_FLOWER,
	BEE_1_WITH_FLOWER,
	FLOWER,
	WALL,
	HIVE_0,
	HIVE_1,
	OUTSIDE,
}

impl CellType {
	pub fn has_flower(&self) -> bool {
		if self.eq(&CellType::BEE_0_WITH_FLOWER) {
			true
		} else if self.eq(&CellType::BEE_1_WITH_FLOWER) {
			true
		} else {
			false
		}
	}

	pub fn is_wall(&self) -> bool {
		if self.eq(&CellType::WALL) {
			true
		} else {
			false
		}
	}

	pub fn is_hive(&self) -> bool {
		if self.eq(&CellType::HIVE_0) || self.eq(&CellType::HIVE_1) {
			true
		} else {
			false
		}
	}
}

impl TryFrom<i32> for CellType {
	type Error = ();

	fn try_from(v: i32) -> Result<Self, Self::Error> {
		match v {
			x if x == CellType::EMPTY as i32 => Ok(CellType::EMPTY),
			x if x == CellType::BEE_0 as i32 => Ok(CellType::BEE_0),
			x if x == CellType::BEE_1 as i32 => Ok(CellType::BEE_1),
			x if x == CellType::BEE_0_WITH_FLOWER as i32 => Ok(CellType::BEE_0_WITH_FLOWER),
			x if x == CellType::BEE_1_WITH_FLOWER as i32 => Ok(CellType::BEE_1_WITH_FLOWER),
			x if x == CellType::FLOWER as i32 => Ok(CellType::FLOWER),
			x if x == CellType::WALL as i32 => Ok(CellType::WALL),
			x if x == CellType::HIVE_0 as i32 => Ok(CellType::HIVE_0),
			x if x == CellType::HIVE_1 as i32 => Ok(CellType::HIVE_1),
			x if x == CellType::OUTSIDE as i32 => Ok(CellType::OUTSIDE),
			_ => Err(()),
		}
	}
}
