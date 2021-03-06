use crate::common::{ AgentInfo, Coords, Action };

#[derive(Debug)]
pub struct Bee {
	pub bee_id: i32,
	pub has_flower: bool,
	pub position: Coords,
	pub destination: Option<Coords>,
	pub target: Option<Coords>,
	pub role: Option<Role>,
	pub path: Option<Vec<Coords>>,
	pub action: Action,
}

impl Bee {
	pub fn new(id: i32, role: Role) -> Self {
		Bee {
			bee_id: id,
			has_flower: false,
			position: Coords { row: 0, col: 0 },
			destination: None,
			target: None,
			role: Some(role),
			path: None,
			action: Bee::get_action_from_role(role.clone()),
		}
	}

	pub fn from_agent_info(info: AgentInfo) -> Self {
		Bee {
			bee_id: info.bee,
			has_flower: false,
			position: Coords { row: info.row as usize, col: info.col as usize },
			destination: None,
			target: None,
			role: None,
			path: None,
			action: Action::MOVE,
		}
	}

	// Current implementation uses view coordinates, not
	// map coordinates.
	pub fn set_destination(&mut self, destination: Coords) {
		self.destination = Some(destination);
	}

	pub fn set_position(&mut self, row: usize, col: usize) {
		self.position.row = row;
		self.position.col = col;
	}

	pub fn set_target(&mut self, target: Option<Coords>) {
		self.target = target;
	}

    pub fn set_role(&mut self, role: Role) {
        self.role = Some(role);
    }

	pub fn at_targets_adjacent(&mut self) -> bool {
		if self.target.is_none() { return false };
		if self.target.unwrap().is_adjacent(&self.position) {
			true
		} else {
			false
		}
	}

	pub fn check_target(&mut self, targets: &Vec<Coords>) {
		if self.target.is_some() {
            if self.role.as_ref().unwrap().eq(&Role::Collect) { return };
			let current = self.target.unwrap();
			let mut i = 0;
			for target in targets {
				if i == self.bee_id { continue };
				i += 1;
				if current.eq(&target) {
					self.target = None
				};
			}
		}
	}

	pub fn get_action_from_role(role: Role) -> Action {
		match role {
			Role::Build => return Action::BUILD,
			Role::Collect => return Action::FORAGE,
            Role::Defender => return Action::FORAGE,
			Role::Sabotage => return Action::GUARD,
		}
	}

    pub fn has_role(&self, role: Role) -> bool {
        match self.role {
            Some(v) => {
                if v.eq(&role) { return true } else { return false };
            },
            None => return false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
	Build,
	Collect,
    Defender,
	Sabotage,
}
