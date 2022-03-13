// This file contains the same functionality as agent.c used in libagent.a

use std::net::TcpStream;
use std::io::Result;
use array2d::Array2D;
use std::io::{ BufRead, BufReader, Write };
use fixed_buffer::FixedBuf;
use crate::common::{
	AgentInfo,
	Command,
	ThinkFunction,
	MAX_COMMAND_LEN,
	NET_BUFFER_SIZE,
	NUM_ROWS,
	NUM_COLS,
	GameState
};
use crate::serialization::{ deserialize_agent_info, serialize_agent_command };
use crate::utils::*;

/// Sends the agent's team name to the arena server.
fn send_team_name(stream: &mut TcpStream, team_name: &String) -> Result<()> {
	let message = team_name.clone().into_bytes();

	if message.len() > NET_BUFFER_SIZE { panic!("Team name is too long") };

	//TODO: terminate team name with newline here instead of in main

	stream.write(&message).expect("Agent unable to send team name to arena.");
	stream.flush()?;
	Ok(())
}

/// Gets a line from the arena server.
fn get_line_from_arena(stream: &mut TcpStream) -> Result<String> {
	let mut reader = BufReader::new(stream);
	let mut buffer: Vec<u8> = Vec::new();
	reader.read_until(b'\n', &mut buffer)?;
	let line = String::from_utf8(buffer).expect("Could not write buffer as string.");
	Ok(line)
}

/// Gets agent info from the arena server and panics if the game is over.
fn get_agent_info(stream: &mut TcpStream) -> Result<AgentInfo> {
	let line = get_line_from_arena(stream).unwrap();

	if line.eq("gameover\n") { panic!("Game over") };

	let info: AgentInfo = deserialize_agent_info(&line);
	Ok(info)
}

/// Sends serialized agent command to the arena server.
fn send_agent_command(command: Command, stream: &mut TcpStream) -> Result<()> {
	let mut buffer: FixedBuf<MAX_COMMAND_LEN> = FixedBuf::new();
	serialize_agent_command(command, &mut buffer);

	let bytes_sent = stream
		.write(&buffer.read_bytes(buffer.len()))
		.expect("Agent unable to send command to arena."); //TODO: Check if bytes sent is 0
	stream.flush()?;
	Ok(())
}

#[allow(unreachable_code)]
/// Main loop that is run for the duration of the game.
fn run(stream: &mut TcpStream, think: ThinkFunction) -> Result<()> {
	println!("Running agent.");
	let mut gamestate = GameState::new();
	let mut heatmap: Array2D<f32>;
	let mut heatmap_initialized = false;
	loop {
		let info: AgentInfo = get_agent_info(stream).expect("Game over.");
		let opponent_col = if info.player == 1 { 3 } else { 27 };
		heatmap = generate_heatmap(NUM_COLS, NUM_ROWS, opponent_col, 13, heatmap_initialized);
		//print_heatmap(&heatmap);
		heatmap_initialized = true;
		//println!("{:?}", info);
		gamestate.update(&info);
		let command: Command = think(&info, &heatmap, &mut gamestate);
		//println!("{:?}", command);
		send_agent_command(command, stream)?;
		//print_map(&gamestate.map);
	}
	unreachable!("The loop should always run");
	Ok(())
}

/// Establishes a Tcp connection with the arena server, sends team name to server
/// and commences the main agent loop.
pub fn agent_main(host: &String, port: &String, team_name: &String, think: ThinkFunction) -> Result<()> {
	let addr = format!("{}:{}", host, port);

	let mut stream = TcpStream::connect(addr).expect("Agent unable to connect to arena.");
	send_team_name(&mut stream, team_name)?;
	run(&mut stream, think)?;

	Ok(())
}
