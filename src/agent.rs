// This file contains the same functionality as agent.c used in libagent.a

use std::net::{ SocketAddr, TcpStream };
use std::io::Result;
use std::io::{ BufRead, BufReader, Read, Write };
use std::time::Duration;
use fixed_buffer::{ deframe_line, FixedBuf, ReadWriteChain };
use crate::common::{ AgentInfo, Command, ThinkFunction, MAX_COMMAND_LEN, NET_BUFFER_SIZE, GameState };
use crate::serialization::{ deserialize_agent_info, serialize_agent_command };


fn send_team_name(stream: &mut TcpStream, team_name: &String) -> Result<()> {
    let message = team_name.clone().into_bytes();

    if message.len() > NET_BUFFER_SIZE { panic!("Team name is too long") };

    //TODO: terminate team name with newline here instead of in main

    stream.write(&message).expect("Agent unable to send team name to arena.");
    stream.flush()?;
    Ok(())
}

fn get_line_from_arena(stream: &mut TcpStream) -> Result<String> {
    let mut reader = BufReader::new(stream);
    let mut buffer: Vec<u8> = Vec::new();
    println!("Attempting to get line from arena");
    reader.read_until(b'\n', &mut buffer)?;
    let line = String::from_utf8(buffer).expect("Could not write buffer as string.");
    Ok(line)
}

fn get_agent_info(stream: &mut TcpStream) -> Result<AgentInfo> {
    let line = get_line_from_arena(stream).unwrap();
    println!("Line from arena: {}", line);

    if line.eq("gameover\n") { panic!("Game over") };

    let info: AgentInfo = deserialize_agent_info(&line);
    println!("{:?}", info);
    Ok(info)
}

fn send_agent_command(command: Command, stream: &mut TcpStream) -> Result<()> {
    let mut buffer: FixedBuf<MAX_COMMAND_LEN> = FixedBuf::new();
    serialize_agent_command(command, &mut buffer);
    // buffer has to be newline terminated

    let bytes_sent = stream
        .write(&buffer.read_bytes(buffer.len()))
        .expect("Agent unable to send command to arena.");
    println!("Bytes sent to arena: {}", bytes_sent);
    stream.flush()?;
    Ok(())
}

#[allow(unreachable_code)]
fn run(stream: &mut TcpStream, think: ThinkFunction) -> Result<()> {
    println!("Running agent.");
    let mut gamestate = GameState::new();
    loop {
        let info: AgentInfo = get_agent_info(stream).expect("Game over.");
        let command: Command = think(&info);
        gamestate.update(&info);
        println!("{:?}", command);
        send_agent_command(command, stream)?;
    }
    unreachable!("The loop should always run");
    Ok(())
}

pub fn agent_main(host: &String, port: &String, team_name: &String, think: ThinkFunction) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    println!("addr: {}", addr);

    let mut stream = TcpStream::connect(addr).expect("Agent unable to connect to arena.");
    send_team_name(&mut stream, team_name)?;
    println!("Team name succesfully sent.");
    run(&mut stream, think)?;

    Ok(())
}
