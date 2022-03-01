// This file contains the same functionality as agent.c used in libagent.a

use std::net::TcpStream;
use std::io::Result;
use std::io::{ Read, Write };
use fixed_buffer::{ deframe_line, FixedBuf, ReadWriteChain };
use crate::common::{ AgentInfo, Command, ThinkFunction, MAX_COMMAND_LEN, NET_BUFFER_SIZE };
use crate::serialization::serialize_agent_command;

fn send_team_name(stream: &mut TcpStream, team_name: &String) -> Result<()> {
    let message = team_name.clone().into_bytes();
    stream.write(&message).expect("Agent unable to send team name to arena.");
    stream.flush()?;
    Ok(())
}

fn get_line_from_arena(stream: &mut TcpStream) -> Result<String> {
    unimplemented!("");
    let mut line = String::new();
    stream.read_to_string(&mut line);
    Ok(line)
}

fn get_agent_info(stream: &mut TcpStream) -> Result<AgentInfo> {
    let line = get_line_from_arena(stream).unwrap();
    let info: AgentInfo = AgentInfo::new();
    Ok(info)
}

fn send_agent_command(command: Command, stream: &mut TcpStream) -> Result<()> {
    let mut buffer: FixedBuf<MAX_COMMAND_LEN> = FixedBuf::new();
    serialize_agent_command(command, &mut buffer);
    // buffer has to be newline terminated
    let bytes_sent = stream
        .write(&buffer.read_bytes(buffer.len()))
        .expect("Agent unable to send command to arena.");
    stream.flush()?;
    Ok(())
}

fn think(info: &AgentInfo) -> Command {
    let command: Command = Command::new();
    command
}

fn run(stream: &mut TcpStream, think_function: ThinkFunction) -> Result<()> {
    loop {
        let info: AgentInfo = get_agent_info(stream).expect("Game over.");
        let command: Command = think_function(info);
        send_agent_command(command, stream);
    }
    Ok(())
}

pub fn agent_main(host: &String, port: &String, team_name: &String, think: ThinkFunction) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    println!("addr: {}", addr);

    let mut stream = TcpStream::connect(addr).expect("Agent unable to connect to arena.");
    send_team_name(&mut stream, team_name);
    run(&mut stream, think);

    Ok(())
}
