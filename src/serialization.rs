// This file contains the same functionality as serialization.c used in libagent.a

use crate::common::{ AgentInfo, Command, MAX_COMMAND_LEN, MAX_AGENT_INFO_LEN };
use fixed_buffer::FixedBuf;

pub fn serialize_agent_info(info: AgentInfo, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    // Implementation of this may be unneeded, seems like only the server uses it
    unimplemented!("");
}

pub fn deserialize_agent_info(buffer: &String) -> AgentInfo {
    let info: AgentInfo = AgentInfo::new();
    //let cells = String::with_capacity(MAX_AGENT_INFO_LEN);

    println!("Serialized agent info in deserialize_agent_info: {}", buffer);
    if buffer.len() < MAX_AGENT_INFO_LEN { panic!("Deserialization fail 0") };

    let params: Vec<&str> = buffer.split(',').collect();
    for param in params.iter() { println!("{}", param) };
    info
}

pub fn serialize_agent_command(command: Command, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    unimplemented!("");
    // Original implementation uses sprintf, writes to buffer using a format
}

pub fn deserialize_agent_command(command: Command, buffer: String) {
    // Implementation of this may be unneeded, seems like only the server uses it
    unimplemented!("");
}
