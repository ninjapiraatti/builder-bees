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

/// The function that decides on a command for a given turn based on agent info.
/// Current logic is similar to the logic in example agent.
pub fn think_simple_agent(info: &AgentInfo, gamestate: &mut GameState) -> Command {
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
