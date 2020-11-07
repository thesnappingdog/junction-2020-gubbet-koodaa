use crate::direction::Direction;
use std::io::{Error as StdError, Read};
use std::net::TcpStream;
use std::str;
use winit::event_loop::EventLoopProxy;

// Events like: echo "okko:connect" | nc localhost 8080
// echo "okko:disconnect" | nc localhost 8080
// echo "okko:left" | nc localhost 8080
pub fn handle_client(
    mut stream: TcpStream,
    event_loop_proxy: &EventLoopProxy<CustomEvent>,
) -> Result<(), StdError> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buf = Vec::with_capacity(256);
    stream
        .read_to_end(&mut buf)
        .expect("Failed to read stream to end");
    let message = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let parts = message.split(":").collect::<Vec<&str>>();
    match parts.len() {
        2 => {
            let name_str = parts[0].split_whitespace().next().unwrap();
            let action_str = parts[1].split_whitespace().next().unwrap();
            match action_str {
                "connect" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerConnected(name_str.to_string()))
                        .ok();
                }
                "disconnect" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerDisconnected(name_str.to_string()))
                        .ok();
                }
                "up" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerMove(name_str.to_string(), Direction::Up))
                        .ok();
                }
                "right" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerMove(
                            name_str.to_string(),
                            Direction::Right,
                        ))
                        .ok();
                }
                "down" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerMove(
                            name_str.to_string(),
                            Direction::Down,
                        ))
                        .ok();
                }
                "left" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerMove(
                            name_str.to_string(),
                            Direction::Left,
                        ))
                        .ok();
                }
                _ => (),
            }
        }
        _ => (),
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub enum CustomEvent {
    PlayerConnected(String),
    PlayerMove(String, Direction),
    PlayerDisconnected(String),
}
