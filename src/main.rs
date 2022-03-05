mod agent;
mod common;
mod serialization;
mod think;
mod bee;
mod simple_agent;
mod utils;

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
use crate::bee::Bee;
use array2d::Array2D;
use crate::think::*;

/// Returns the hive cell type given a player number.
pub fn hive_cell(player: i32) -> Cell {
	if player == 0 {
		Cell::HIVE_0
	} else {
		Cell::HIVE_1
	}
}

/// The function that decides on a command for a given turn based on agent info.
/// Current logic is similar to the logic in example agent.
pub fn think(info: &AgentInfo, heatmap: &Array2D<f32>, gamestate: &mut GameState) -> Command {
	let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });

	// If the current bee holds a flower, check if the hive is adjacent
	// and forage the flower to the hive if possible.
	if bee_cell.has_flower() {
		let hive_direction = find_neighbour(info, &hive_cell(info.player));
		match hive_direction {
			Some(v) => return Command {
				action: Action::FORAGE,
				direction: v,
			},
			None => (),
		}
	// If the current bee doesn't hold a flower, check if there is a flower
	// adjacent to the bee. Pick up if possible.
	} else {
		let wall_direction = find_heat(info, &heatmap);
		//let flower_direction = find_neighbour(info, &Cell::FLOWER);
		match wall_direction {
			Some(v) => return Command {
				action: Action::MOVE,
				direction: v,
			},
			None => (),
		}
	}

	// Otherwise move in a random direction.
	let random_direction: Direction = rand::random();
	Command {
		action: Action::MOVE,
		direction: random_direction,
	}
}

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
