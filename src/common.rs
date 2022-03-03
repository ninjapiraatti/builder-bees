#![allow(unused)]
#![allow(non_camel_case_types)]

use array2d::Array2D;
use std::fmt::{ Debug, Formatter };
use std::fmt;

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

pub type ThinkFunction = fn(&AgentInfo) -> Command;

pub struct AgentInfo {
    pub turn: i32,
    pub player: i32,
    pub bee: i32,
    pub row: i32,
    pub col: i32,
    pub cells: Array2D<Cell>,
}

pub struct Map {
    pub cells: Array2D<Cell>,
    //pub width: i32,
    //pub height: i32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: Array2D::filled_with(Cell::EMPTY, NUM_COLS, NUM_ROWS),
            //width: NUM_COLS,
            //height: NUM_ROWS,
        }
    }
}


pub struct GameState {
    pub map: Map,
    pub strategy: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            strategy: STRATEGY_BUILD_WALLS,
        }
    }

    pub fn update(&mut self, agent_info: &AgentInfo) {
        for row in 0..VIEW_SIZE {
            //print!("\n");
            for col in 0..VIEW_SIZE {
                //print!("{:?}", self.map.cells.get(row,col).unwrap());
                if col >= 0 && col < NUM_COLS && row >= 0 && row < NUM_ROWS {
                    let y = agent_info.row + row as i32 - VIEW_DISTANCE as i32;
                    let x = agent_info.col + col as i32 - VIEW_DISTANCE as i32;
                    self.map.cells.set(row, col, Cell::from(*agent_info.cells.get(x as usize, y as usize).unwrap()));
                }
            }
        }
    }
}

impl AgentInfo {
    pub fn new() -> Self {
        Self {
            turn: 0,
            player: 0,
            bee: 0,
            row: 0,
            col: 0,
            cells: Array2D::filled_with(Cell::EMPTY, VIEW_SIZE, VIEW_SIZE),
        }
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

pub struct Coords {
    row: i32,
    col: i32,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Action {
    MOVE,
    FORAGE,
    BUILD,
    GUARD,
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
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

impl TryFrom<i32> for Cell {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Cell::EMPTY as i32 => Ok(Cell::EMPTY),
            x if x == Cell::BEE_0 as i32 => Ok(Cell::BEE_0),
            x if x == Cell::BEE_1 as i32 => Ok(Cell::BEE_1),
            x if x == Cell::BEE_0_WITH_FLOWER as i32 => Ok(Cell::BEE_0_WITH_FLOWER),
            x if x == Cell::BEE_1_WITH_FLOWER as i32 => Ok(Cell::BEE_1_WITH_FLOWER),
            x if x == Cell::FLOWER as i32 => Ok(Cell::FLOWER),
            x if x == Cell::WALL as i32 => Ok(Cell::WALL),
            x if x == Cell::HIVE_0 as i32 => Ok(Cell::HIVE_0),
            x if x == Cell::HIVE_1 as i32 => Ok(Cell::HIVE_1),
            x if x == Cell::OUTSIDE as i32 => Ok(Cell::OUTSIDE),
            _ => Err(()),
        }
    }
}
