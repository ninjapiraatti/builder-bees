use array2d::Array2D;
use crate::common::{
    Map,
    CellType,
    Coords,
    Direction,
    CellType::*,
    NUM_ROWS,
    NUM_COLS
};

#[allow(dead_code)]
pub fn print_map(map: &Map) {
	for row in 0..NUM_ROWS {
		print!("\n");
		for col in 0..NUM_COLS {
			let chr = map.cells.get(row, col);
            let mut celltype = CellType::EMPTY;
            match chr {
                Some(v) => celltype = v.celltype,
                None => (),
            }
			match celltype {
				EMPTY => print!(".  "),
				BEE_0 => print!("b  "),
				BEE_1 => print!("B  "),
				BEE_0_WITH_FLOWER => print!("bf "),
				BEE_1_WITH_FLOWER => print!("Bf "),
				FLOWER => print!("*  "),
				HIVE_0 => print!("0  "),
				HIVE_1 => print!("1  "),
				WALL => print!("#  "),
				OUTSIDE => print!("X  "),
			}
		}
	}
}

pub fn generate_heatmap(width: usize, height: usize, originx: usize, originy: usize, initialized: bool) -> Array2D<f32> {
	let mut heatmap = Array2D::filled_with(100.0, height, width);
	if initialized {
		for row in 0..height {
			for col in 0..width {
				let x = col as i32 - originx as i32;
				let y = row as i32 - originy as i32;
				heatmap.set(row, col, ((x*x + y*y) as f32).sqrt()).expect("Could not set heatmap value");
			}
		}
	}
	heatmap
}

#[allow(dead_code)]
pub fn print_heatmap(heatmap: &Array2D<f32>) {
	for row in 0..heatmap.num_rows() {
		print!("\n");
		for col in 0..heatmap.num_columns() {
			print!("{:.2}  ", heatmap.get(row, col).unwrap_or_else(|| &100.0));
		}
	}
}

pub fn coords_to_dir(current: Coords, target: Coords) -> Direction {
	let x = target.col as i32 - current.col as i32;
	let y = target.row as i32 - current.row as i32;
	if x == 0 {
		if y > 0 {
			return Direction::S;
		} else {
			return Direction::N;
		}
	}
	if y == 0 {
		if x > 0 {
			return Direction::E;
		} else {
			return Direction::W;
		}
	}
	if x > 0 && y > 0 {
		return Direction::SE;
	}
	if x > 0 && y < 0 {
		return Direction::NE;
	}
	if x < 0 && y > 0 {
		return Direction::SW;
	}
	if x < 0 && y < 0 {
		return Direction::NW;
	}
	Direction::N 
}
