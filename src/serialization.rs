// This file contains the same functionality as serialization.c used in libagent.a

use crate::common::{ AgentInfo, Command, MAX_COMMAND_LEN, MAX_AGENT_INFO_LEN, NET_BUFFER_SIZE };
use fixed_buffer::FixedBuf;

pub fn serialize_agent_info(info: AgentInfo, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    // Implementation of this may be unneeded, seems like only the server uses it
    unimplemented!("");
}

pub fn deserialize_agent_info(buffer: &String) -> AgentInfo {
    //let info: AgentInfo = AgentInfo::new();
    //let cells = String::with_capacity(MAX_AGENT_INFO_LEN);

    println!("Serialized agent info in deserialize_agent_info: {}", buffer);
    println!("MAX_AGENT_INFO_LEN: {}", MAX_AGENT_INFO_LEN);
    println!("Buffer len: {}", buffer.len());
    if buffer.len() >= MAX_AGENT_INFO_LEN { panic!("Deserialization fail 0") };

    let params: Vec<&str> = buffer.split(',').collect();
    for param in params.iter() { println!("{}", param) };
    AgentInfo {
        turn: params[0].parse::<i32>().unwrap(),
        player: params[1].parse::<i32>().unwrap(),
        bee: params[2].parse::<i32>().unwrap(),
        row: params[3].parse::<i32>().unwrap(),
        col: params[4].parse::<i32>().unwrap(),
        ..AgentInfo::new() //TODO: Implement function for String -> Array2D
    }
}

pub fn serialize_agent_command(command: Command, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    let message: String = format!("{},{}\n",
                                  command.action as i32,
                                  command.direction as i32);
    buffer.write_str(&message).expect("Command string could not be written to buffer.");
    println!("Command string written to buffer.");
    println!("Buffer: {:?}", buffer);
}

pub fn deserialize_agent_command(command: Command, buffer: String) {
    // Implementation of this may be unneeded, seems like only the server uses it
    unimplemented!("");
}
