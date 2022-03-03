// This file contains the same functionality as serialization.c used in libagent.a

use crate::common::{ 
    AgentInfo, 
    Cell, 
    Command, 
    MAX_COMMAND_LEN, 
    MAX_AGENT_INFO_LEN, 
    VIEW_SIZE,
};
use fixed_buffer::FixedBuf;
use array2d::Array2D;

/// Helper function that converts the cells portion of the serialized agent info string into
/// a 2D array of Cells
fn parse_cells_string(string: &str) -> Array2D<Cell> {
    let mut cell_chars: Vec<char> = string.chars().collect();
    cell_chars.pop();
    let cells_i32: Vec<i32> = cell_chars.iter()
        .map(|&x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let cells: Vec<Cell> = cells_i32.iter().map(|&x| x.try_into().unwrap()).collect();
    let array: Array2D<Cell> = Array2D::from_row_major(&cells, VIEW_SIZE, VIEW_SIZE);
    array
}

/// Deserializes the agent info string and returns an AgentInfo
pub fn deserialize_agent_info(buffer: &String) -> AgentInfo {
    if buffer.len() >= MAX_AGENT_INFO_LEN { panic!("Deserialization fail 0") };

    let params: Vec<&str> = buffer.split(',').collect();
    for param in params.iter() { println!("{}", param) };
    AgentInfo {
        turn: params[0].parse::<i32>().unwrap(),
        player: params[1].parse::<i32>().unwrap(),
        bee: params[2].parse::<i32>().unwrap(),
        row: params[3].parse::<i32>().unwrap(),
        col: params[4].parse::<i32>().unwrap(),
        cells: parse_cells_string(params[5]),
    }
}

/// Serializes a Command and returns and saves the result into a designated buffer.
pub fn serialize_agent_command(command: Command, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    let message: String = format!("{},{}\n",
                                  command.action as i32,
                                  command.direction as i32);
    buffer.write_str(&message).expect("Command string could not be written to buffer.");
}
