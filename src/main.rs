mod agent;
mod common;
mod serialization;

use std::env;
use crate::common::{ AgentInfo, Command };

pub fn think(info: &AgentInfo) -> Command {
    let command: Command = Command::new();
    command
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: ./agent arena_host arena_ip");
    }

    let host: &String = args.get(1).unwrap();
    let port: &String = args.get(2).unwrap();
    let team_name = "builder-bees\n".to_string();

    agent::agent_main(host, port, &team_name, think).expect("Program should didn't exit in agent_main");
}
