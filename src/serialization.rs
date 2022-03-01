// This file contains the same functionality as serialization.c used in libagent.a

use crate::common::{ AgentInfo, Command, MAX_COMMAND_LEN };
use fixed_buffer::FixedBuf;

pub fn serialize_agent_info(info: AgentInfo, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    unimplemented!("");
}

pub fn deserialize_agent_info(buffer: String) -> AgentInfo {
    unimplemented!("");
}

pub fn serialize_agent_command(command: Command, buffer: &mut FixedBuf<MAX_COMMAND_LEN>) {
    unimplemented!("");
    // Original implementation uses sprintf, writes to buffer using a format
}

pub fn deserialize_agent_command(command: Command, buffer: String) {
    unimplemented!("");
}
