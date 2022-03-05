mod agent;
mod common;
mod serialization;
mod think;
mod bee;
mod simple_agent;

use std::env;
use crate::common::{ 
    Action,
    AgentInfo,
    Cell,
    Command,
    Coords,
    Direction,
    GameState,
    NUM_COLS,
    NUM_ROWS,
    VIEW_DISTANCE
};
use crate::think::{
    find_neighbour,
    find_flower_in_view,
    get_direction_to_destination,
    hive_coords,
    hive_cell
};
use crate::bee::Bee;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: ./agent arena_host arena_ip");
    }

    let host: &String = args.get(1).unwrap();
    let port: &String = args.get(2).unwrap();
    let team_name = "builder-bees\n".to_string();

    agent::agent_main(host, port, &team_name, think).expect("Program should not exit in agent_main");
}
//TODO: Doesn't work with hive_coords because hive_coords gives map coords
