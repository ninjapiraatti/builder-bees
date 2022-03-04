mod agent;
mod common;
mod serialization;
mod think;
mod bee;

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
use crate::think::{ find_neighbour, find_flower_in_view };
use crate::bee::Bee;

/// Returns the hive cell type given a player number.
pub fn hive_cell(player: i32) -> Cell {
    if player == 0 {
        Cell::HIVE_0
    } else {
        Cell::HIVE_1
    }
}

/// Returns the hive coords type given a player number.
pub fn hive_coords(player: i32) -> Coords {
    if player == 0 {
        Coords { row: 12, col: 1 }
    } else {
        Coords { row: 12, col: NUM_COLS - 1 }
    }
}

//TODO: Doesn't work with hive_coords because hive_coords gives map coords
fn get_direction_to_destination(destination: &Coords) -> Direction {
    match destination {
        &Coords { row, col } if row < VIEW_DISTANCE && col == VIEW_DISTANCE => Direction::N,
        &Coords { row, col } if row < VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::NE,
        &Coords { row, col } if row == VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::E,
        &Coords { row, col } if row > VIEW_DISTANCE && col > VIEW_DISTANCE => Direction::SE,
        &Coords { row, col } if row > VIEW_DISTANCE && col == VIEW_DISTANCE => Direction::S,
        &Coords { row, col } if row > VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::SW,
        &Coords { row, col } if row == VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::W,
        &Coords { row, col } if row < VIEW_DISTANCE && col < VIEW_DISTANCE => Direction::NW,
        &_ => Direction::S, //TODO:Fix this
    }
}

/// The function that decides on a command for a given turn based on agent info.
/// Current logic is similar to the logic in example agent.
pub fn think(info: &AgentInfo, gamestate: &mut GameState) -> Command {
    let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });
    let current_bee = &mut gamestate.bees[info.bee as usize];

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
        let flower_direction = find_neighbour(info, &Cell::FLOWER);
        match flower_direction {
            Some(v) => return Command {
                action: Action::FORAGE,
                direction: v,
            },
            None => (),
        }
    }

    // If the current bee doesn't have a flower, and there was nothing to forage
    // but the bee has a destination or a flower is within view, move towards
    // that destination or set that flower as destination.
    if !bee_cell.has_flower() {
        if current_bee.destination.is_some() {
            let bee_destination: &Coords = current_bee.destination.as_ref().unwrap();
            let direction = get_direction_to_destination(bee_destination);
            return Command {
                action: Action::MOVE,
                direction: direction,
            }
        } else {
            let flower_coords = find_flower_in_view(info);
            match flower_coords {
                Some(v) => gamestate.bees[info.bee as usize].set_destination(v),
                None => (),
            }
        }
    //TODO: When bee has flower, move towards hive.
    } else {
        current_bee.destination = None;
        let hive_direction = get_direction_to_destination(&hive_coords(info.player));
        return Command {
            action: Action::MOVE,
            direction: hive_direction,
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
//TODO: Doesn't work with hive_coords because hive_coords gives map coords
