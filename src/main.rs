mod agent;
mod common;
mod serialization;
mod think;
mod bee;
mod simple_agent;
mod utils;

use std::env;
use std::process;
use crate::common::{ 
    Action,
    AgentInfo,
    CellType,
    Command,
    Coords,
    Direction,
    GameState,
    Pos,
    NUM_COLS,
    NUM_ROWS,
    VIEW_DISTANCE
};
use array2d::Array2D;
use crate::think::*;
use crate::simple_agent::*;
use crate::utils::coords_to_dir;
use pathfinding::astar;

/// The main think function of our agent. 
pub fn think(info: &AgentInfo, heatmap: &Array2D<f32>, gamestate: &mut GameState) -> Command {
	let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });

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
		let opponent_col = if info.player == 1 { 2 } else { NUM_COLS - 3 };
		//let pathfind_direction = pathfind(info, &gamestate.map, &Coords { row: 9, col: opponent_col });
        let goal = Pos(9, opponent_col as i32);
		let path= astar(&Pos(info.row as i32, info.col as i32), |p| p.neighbours(&gamestate.map), |p| p.distance(&goal), |p| *p == goal);
		//println!("\x1b[96mdir: {:?}\x1b[0m", pathfind_direction);
		match path {
			Some(v) => {
                let pos = v.0.get(1).unwrap();
                let next = Coords { row: pos.0 as usize, col: pos.1 as usize };
                let current = Coords { row: info.row as usize, col: info.col as usize };
                return Command {
				    action: Action::MOVE,
				    direction: coords_to_dir(current, next),
                }
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
