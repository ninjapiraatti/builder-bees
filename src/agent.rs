// This file contains the same functionality as agent.c used in libagent.a

use std::net::TcpStream;
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

pub fn agent_main(host: &String, port: &String, team_name: &String, think: ThinkFunction) -> i32 {
    let addr = format!("{}:{}", host, port);
    println!("addr: {}", addr);

    if let Ok(stream) = TcpStream::connect(addr) {
        println!("Succesful connection to arena");
    } else {
        panic!("Could not connect to arena");
    }

    //send_team_name();
    //run();

    //close_socket();
    0
}
