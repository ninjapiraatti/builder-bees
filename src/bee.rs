use crate::common::{ AgentInfo, Coords };

#[derive(Debug)]
pub struct Bee {
	pub bee_id: i32,
	pub has_flower: bool,
	pub position: Coords,
	pub destination: Option<Coords>,
	pub target: Option<Coords>,
	pub role: Option<Role>,
	pub path: Option<Vec<Coords>>,
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

	pub fn at_target(&mut self) -> bool {
		if self.target.is_none() { return false };
		if self.target.unwrap().is_adjacent(&self.position) {
			true
		} else {
			false
		}
	}

	pub fn check_target(&mut self, targets: &Vec<Coords>) {
		if self.target.is_some() {
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
}

#[derive(Debug, PartialEq)]
pub enum Role {
	Collect,
	Build,
	Sabotage,
}
