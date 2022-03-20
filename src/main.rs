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
use crate::utils::{ coords_to_dir, print_heatmap };
use crate::pathfind::*;

/// The main think function of our agent. 
pub fn think(info: &AgentInfo, heatmap: &Array2D<f32>, gamestate: &mut GameState) -> Command {
	let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });
	let targets = gamestate.get_targets();
	let mut bee = gamestate.bees.get_mut(info.bee as usize).unwrap();
	bee.set_position(info.row as usize, info.col as usize);
	bee.check_target(&targets);
	//println!("\x1b[96m\nThere are {:?} targets: {:?}\x1b[0m", targets.len(), targets);
	//println!("\x1b[96mBee {:?} target: {:?}\x1b[0m", bee.bee_id, bee.target);


	//TODO: Function that checks if strategy has to be changed.

	// If the current bee holds a flower, check if the hive is adjacent
	// and forage the flower to the hive if possible.
	//let test_coords = Coords { row: 5, col: 5 };
	if bee.bee_id == 4 {
		println!("\x1b[96m\nBee {:?}\x1b[0m", bee.bee_id);
	}
	bee.has_flower = bee_cell.has_flower(); 
	if bee.has_flower == true {
		let hive = Some(hive_coords(info.player)).unwrap();
		bee.target = find_available_adjacent(hive, &gamestate.map.cells);
		let hive_direction = find_neighbour(info, &hive_cell(info.player));
		match hive_direction {
			Some(v) => {
				println!("\x1b[93mBee {:?} has a flower. Target: {:?} Bee coords: {:?} Hive coords: {:?}\x1b[0m", bee.bee_id, bee.target, bee.position, hive);
				return Command {
				action: Action::FORAGE,
				direction: v,
				};
			},
			None => (),
		}
	} else {
		if bee.role.as_ref().unwrap().eq(&Role::Collect) {
			let flower_direction = find_neighbour(info, &CellType::FLOWER);
			if flower_direction.is_some() {
				let command = Command {
					action: bee.action,
					direction: flower_direction.unwrap(),
				};
				bee.has_flower = true;
				return command;
			}
		}
	}

	// Is the bee adjacent to its target? If so, do the action.
	if bee.at_targets_adjacent() && bee.bee_id != 4 {
		if bee.bee_id == 4 {
			//println!("\x1b[96mthink 66: Bee {:?} is at target. target: {:?}\x1b[0m", bee.bee_id, bee.target);
			//println!("\x1b[96mthink 67: Bee {:?} is at {:?}\x1b[0m", bee.bee_id, bee.position);
		}
		let command = Command {
			action: bee.action,
			direction: get_direction(bee.target.as_ref().unwrap(), &bee.position).unwrap(),
		};
		bee.set_target(None);
		if bee.bee_id == 4 {
			//println!("\x1b[96mthink 75: Returning command {:?}\x1b[0m", command);
		}
		return command;
	}

	// If the bee has no target, get one
	if bee.target.as_ref().is_none() {
		//println!("\x1b[96mBee {:?} has no target. \x1b[0m", bee.bee_id);
		let target = find_target(info, &bee, heatmap, &gamestate.map.cells, &targets);
		if target.is_some() {
			bee.set_target(target);
			if bee.bee_id == 4 {
				//println!("\x1b[96mthink 87: Bee {:?} now has target: {:?}. \x1b[0m", bee.bee_id, target);
			}
		}
	}

	// If it's a builder bee, move towards target
	if bee.role.as_ref().unwrap().eq(&Role::Build) {
		//println!("\x1b[96mBee {:?} is a builder. \x1b[0m", bee.bee_id);
		let opponent_col = if info.player == 1 { 2 } else { NUM_COLS - 3 };
		let opponent_hive = Coords { row: 9, col: opponent_col };
		let command: Option<Command> = pathfind(info, &gamestate.map, bee.target.as_ref().unwrap_or(&opponent_hive));
		match command {
			Some(v) => return v,
			None => (),
		}
	}

	// If it's a forager bee, move towards target
	if bee.role.as_ref().unwrap().eq(&Role::Collect) {
		//println!("\x1b[96mBee {:?} is a collector. \x1b[0m", bee.bee_id);
		let home_hive = hive_coords(info.player);
		let command: Option<Command> = pathfind(info, &gamestate.map, bee.target.as_ref().unwrap_or(&home_hive));
		if bee.bee_id == 4 {
			//println!("\x1b[96mBee target to pathfind: {:?} | Returns command: {:?}. \x1b[0m", bee.target, command);
		}
		match command {
			Some(v) => return v,
			None => (),
		}
	}

	// Otherwise move in a random direction.
	let random_direction: Direction = rand::random();
	//println!("\x1b[96mBee {:?} is moving at random. \x1b[0m", bee.bee_id);
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
