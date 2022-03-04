use crate::common::{ AgentInfo, Coords };

pub struct Bee {
    bee_id: i32,
    has_flower: bool,
    position: Coords,
    pub destination: Option<Coords>,
    role: Option<Role>,
}

impl Bee {
    pub fn new(id: i32) -> Self {
        Bee {
            bee_id: id,
            has_flower: false,
            position: Coords { row: 0, col: 0 },
            destination: None,
            role: None,
        }
    }

    pub fn from_agent_info(info: AgentInfo) -> Self {
        Bee {
            bee_id: info.bee,
            has_flower: false,
            position: Coords { row: info.row as usize, col: info.col as usize },
            destination: None,
            role: None,
        }
    }

    // Current implementation uses view coordinates, not
    // map coordinates.
    pub fn set_destination(&mut self, destination: Coords) {
        self.destination = Some(destination);
    }
}

pub enum Role {
    CollectNearest,
    Sabotage,
}
