// This file, agent.rs, corresponds to agent.c and agent.h of the hive-arena repository.

pub struct AgentInfo {
    turn: i32,
    player: i32,
    bee: i32,
    row: i32,
    col: i32,
}

pub struct Command {
    action: Action,
    direction: Direction,
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

type think_function = fn(AgentInfo) -> Command;

fn connect_to_arena(host. String, port: i32) -> i32 {
    unimplemented!("Establishes TCP connection to arena, returns socket");
}

fn send_team_name(socket: i32, name: String) {
    unimplemented!("Sends the team name to arena at beginning of game");
}

fn close_socket(socket: i32) {
    unimplemented!("closes socket connection");
}

fn send_agent_command(command: Command, socket: i32) {
    unimplemented!("sends agent command to arena");
}

fn run(socket: i32, 
pub fn agent_main(host: String, port: i32, team_name: String, command: Command) -> i32 {
    unimplemented!("Once called, loops until the end of the game");
}
