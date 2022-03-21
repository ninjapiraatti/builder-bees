use crate::common::{ 
    Action,
    AgentInfo,
    CellType,
    Command,
    Coords,
    Direction,
    GameState,
    VIEW_DISTANCE
};
use crate::think::{
    can_move_in_direction,
    find_neighbour,
    find_flower_in_view,
    get_direction,
    hive_coords,
    hive_cell
};
use array2d::Array2D;

/// The function that decides on a command for a given turn based on agent info.
/// Current logic is similar to the logic in example agent.
#[allow(dead_code, unused_variables)]
pub fn think_simple_agent(info: &AgentInfo, heatmap: &Array2D<f32>, gamestate: &mut GameState) -> Command {
    let bee_cell = info.cell_type(&Coords { row: VIEW_DISTANCE, col: VIEW_DISTANCE });
    let current_bee = &mut gamestate.bees[info.bee as usize];
    let bee_position = Coords { row: info.row as usize, col: info.col as usize };

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
        let flower_direction = find_neighbour(info, &CellType::FLOWER);
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
            let direction = get_direction(bee_destination, &bee_position);
            match direction {
                Some(v) if can_move_in_direction(info, &v) => return Command {
                    action: Action::MOVE,
                    direction: v,
                },
                Some(_) => (),
                None => (),
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
        let hive_direction = get_direction(&hive_coords(info.player), &bee_position);
        match hive_direction {
            Some(v) if can_move_in_direction(info, &v) => return Command {
                action: Action::MOVE,
                direction: v,
            },
            Some(_) => (),
            None => (),
        }
    }
    // Otherwise move in a random direction.
    let mut random_direction: Direction = rand::random();
    while !can_move_in_direction(info, &random_direction) {
        random_direction = rand::random();
    }
    Command {
        action: Action::MOVE,
        direction: random_direction,
    }
}
