// This file contains the same functionality as agent.c used in libagent.a

use std::net::TcpStream;
use std::io::Result;
use std::io::Write;
use crate::common::{ AgentInfo, Command, ThinkFunction };

fn connect_to_arena(host: &String, port: i32) -> i32 {
    unimplemented!("Establishes TCP connection to arena, returns socket");
}

fn send_team_name(socket: i32, name: String) {
    unimplemented!("Sends the team name to arena at beginning of game");
}

fn close_socket(socket: i32) {
    unimplemented!("closes socket connection");
}

fn send_agent_command(command: Command, socket: i32) {
    unimplemented!("sends agent command to arena");
}

pub fn agent_main(host: &String, port: &String, team_name: &String, think: ThinkFunction) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    println!("addr: {}", addr);

    let mut stream = TcpStream::connect(addr).expect("Agent unable to connect to arena.");
    let message = team_name.clone().into_bytes();
    stream.write(&message).expect("Agent unable to send message.");
    stream.flush()?;

    Ok(())
}
