use crate::common::{ AgentInfo, Coords };

pub struct Bee {
    //bee_id: i32,
    has_flower: bool,
    position: Coords,
    destination: Coords,
    role: Role,
}

impl Bee {
    pub fn new(info: AgentInfo) -> Self {
        has_flower: false,
        position: Coords { info.row, info.col },
        destination: Coords { info.row, 15 },
        role: Role::CollectNearest,
    }
}

pub enum Role {
    CollectNearest,
    Sabotage,
}
