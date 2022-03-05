use crate::common::{ Map, Cell::*, NUM_ROWS, NUM_COLS };

pub fn print_map(map: &Map) {
  for row in 0..NUM_ROWS {
    print!("\n");
    for col in 0..NUM_COLS {
      let chr = map.cells.get(row,col).unwrap_or_else(|| &EMPTY);
      match chr {
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