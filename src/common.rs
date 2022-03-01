pub const NUM_ROWS: i32 = 25;
pub const NUM_COLS: i32 = 30;

pub const NUM_PLAYERS: i32 = 2;
pub const NUM_BEES: i32 = 5;

pub const TURNS_BEFORE_TIMEOUT: i32 = 1000;

pub const VIEW_DISTANCE: i32 = 3;
pub const VIEW_SIZE: i32 = VIEW_DISTANCE * 2 + 1;

pub const MAX_COMMAND_LEN: usize = 10;
pub const NET_BUFFER_SIZE: usize = 200;

pub type ThinkFunction = fn(AgentInfo) -> Command;

pub struct AgentInfo {
    turn: i32,
    player: i32,
    bee: i32,
    row: i32,
    col: i32,
}

impl AgentInfo {
    pub fn new() -> Self {
        Self {
            turn: 0,
            player: 0,
            bee: 0,
            row: 0,
            col: 0,
        }
    }
}

pub struct Command {
    action: Action,
    direction: Direction,
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

pub enum Action {
    MOVE,
    FORAGE,
    BUILD,
    GUARD,
}

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
