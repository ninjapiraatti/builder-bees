mod agent;
mod common;
mod serialization;
mod think;
mod bee;
mod simple_agent;
mod utils;
mod pathfind;

use std::env;
use std::process;
use array2d::Array2D;
use crate::common::{ 
    Action,
    AgentInfo,
    CellType,
    Command,
    Coords,
    Direction,
    GameState,
    NUM_COLS,
    NUM_ROWS,
    VIEW_DISTANCE
};
use crate::bee::*;
use crate::think::*;
use crate::simple_agent::*;
use crate::utils::coords_to_dir;
use crate::pathfind::*;

/// The main think function of our agent. 
pub fn think(info: &AgentInfo, heatmap: &Array2D<f32>, gamestate: &mut GameState) -> Command {
	let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });
    let mut bee = gamestate.bees.get_mut(info.bee as usize).unwrap();
    bee.set_position(info.row as usize, info.col as usize);


	//TODO: Function that checks if strategy has to be changed.

	// If the current bee holds a flower, check if the hive is adjacent
	// and forage the flower to the hive if possible.
	//let test_coords = Coords { row: 5, col: 5 };
	if bee_cell.has_flower() {
		let hive_direction = find_neighbour(info, &hive_cell(info.player));
		match hive_direction {
			Some(v) => return Command {
				action: Action::FORAGE,
				direction: v,
			},
			None => (),
		}
	// If the current bee doesn't hold a flower, find the direction in which
    // a wall should be built, and move toward it.
	} else {
		/*
		let wall_direction = find_heat(info, &heatmap);
		//let flower_direction = find_neighbour(info, &CellType::FLOWER);
		match wall_direction {
			Some(v) => return Command {
				action: Action::MOVE,
				direction: v,
			},
			None => (),
		}*/
        if bee.at_target() {
            return Command {
                action: Action::BUILD,
                direction: get_direction(bee.target.as_ref().unwrap(), &bee.position).unwrap(),
            }
        }
        if bee.target.as_ref().is_none() {
            let target = find_target(info, heatmap);
            if target.is_some() {
                bee.set_target(target.unwrap());
            }
        }
        if bee.role.as_ref().unwrap().eq(&Role::Build) {
	        let opponent_col = if info.player == 1 { 2 } else { NUM_COLS - 3 };
            let opponent_hive = Coords { row: 9, col: opponent_col };
            let command: Option<Command> = pathfind(info, &gamestate.map, bee.target.as_ref().unwrap_or(&opponent_hive));
            match command {
		    	Some(v) => return v,
		    	None => (),
            }
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
