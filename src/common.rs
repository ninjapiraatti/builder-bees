#![allow(unused)]
#![allow(non_camel_case_types)]

use array2d::Array2D;
use std::fmt::{ Debug, Formatter, Result };

pub const NUM_ROWS: i32 = 25;
pub const NUM_COLS: i32 = 30;

pub const NUM_PLAYERS: i32 = 2;
pub const NUM_BEES: i32 = 5;

pub const TURNS_BEFORE_TIMEOUT: i32 = 1000;

pub const VIEW_DISTANCE: usize = 3;
pub const VIEW_SIZE: usize = VIEW_DISTANCE * 2 + 1;

pub const MAX_AGENT_INFO_LEN: usize = (VIEW_SIZE * VIEW_SIZE + 30);
pub const MAX_COMMAND_LEN: usize = 10;
pub const NET_BUFFER_SIZE: usize = 200;

pub type ThinkFunction = fn(&AgentInfo) -> Command;

pub struct AgentInfo {
    pub turn: i32,
    pub player: i32,
    pub bee: i32,
    pub row: i32,
    pub col: i32,
    pub cells: Array2D<Cell>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

#[derive(Clone)]
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
