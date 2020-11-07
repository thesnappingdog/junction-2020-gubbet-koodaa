use crate::direction::Direction;
use crate::game::MazeGame;
use crate::gui::Gui;
use crate::window::AppWindow;
use log::error;
use pixels::Error;
use std::io::{Error as StdError, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::time::Instant;
use uuid::Uuid;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use winit_input_helper::WinitInputHelper;

// Events like: echo "connect:okko" | nc localhost 8080
// echo "disconnect:okko" | nc localhost 8080
// echo "move:okko:left" | nc localhost 8080
fn handle_client(
    mut stream: TcpStream,
    event_loop_proxy: &EventLoopProxy<CustomEvent>,
) -> Result<(), StdError> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut buf = Vec::with_capacity(256);
    stream.read_to_end(&mut buf);
    let message = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let parts = message.split(":").collect::<Vec<&str>>();
    match parts.len() {
        2 => {
            let action_str = parts[0].split_whitespace().next().unwrap();
            let name_str = parts[1].split_whitespace().next().unwrap();
            match action_str {
                "connect" => {
                    println!("try to send connect event {}", name_str);
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerConnected(name_str.to_string()))
                        .ok();
                }
                "disconnect" => {
                    event_loop_proxy
                        .send_event(CustomEvent::PlayerDisconnected(name_str.to_string()))
                        .ok();
                }
                _ => (),
            }
        }
        3 => {
            let action_str = parts[0].split_whitespace().next().unwrap();
            let name_str = parts[1].split_whitespace().next().unwrap();
            let dir = parts[2].split_whitespace().next().unwrap();
            match action_str {
                "move" => {
                    let direction = match dir {
                        "up" => Some(Direction::Up),
                        "right" => Some(Direction::Right),
                        "down" => Some(Direction::Down),
                        "left" => Some(Direction::Left),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        event_loop_proxy
                            .send_event(CustomEvent::PlayerMove(name_str.to_string(), direction))
                            .ok();
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }
    println!("{:?}", message);
    Ok(())
}

#[derive(Debug, Clone)]
pub enum CustomEvent {
    PlayerConnected(String),
    PlayerMove(String, Direction),
    PlayerDisconnected(String),
}

pub struct App {
    dt: f64,
    fps: f64,
}

impl App {
    pub fn build() -> App {
        App { dt: 0., fps: 0. }
    }

    pub fn run(mut self, name: &str, width: u32, height: u32, maze_size: i32) -> Result<(), Error> {
        let mut time = Instant::now();
        let event_loop = EventLoop::<CustomEvent>::with_user_event();
        let mut window = AppWindow::new(name, &event_loop, width, height);
        let mut input = WinitInputHelper::new();
        let mut frame_sum = 0.;
        let mut dt_sum = 0.;
        let mut gui = Gui::new(&window.window(), &window.pixels());
        let mut game = MazeGame::new(maze_size, &window);
        let event_loop_proxy = event_loop.create_proxy();
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
        // Listen player connections
        std::thread::spawn(move || loop {
            for stream in listener.incoming() {
                handle_client(stream.expect("Failed to read stream"), &event_loop_proxy);
            }
        });
        event_loop.run(move |event, _, control_flow| {
            gui.handle_event(&window.window(), &event, &mut game);
            if let Event::RedrawRequested(_) = event {
                window.clear().expect("Failed to clear");
                game.update(&mut window, self.dt);
                if window
                    .present_with_gui(&mut gui)
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                self.capture_fps(&mut dt_sum, &mut frame_sum, &mut time);
            }
            game.handle_custom_events(&event);
            if input.update(&event) {
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                game.handle_input(&window, &input);
                if let Some(size) = input.window_resized() {
                    window.resize(size);
                }
            }
            window.request_redraw();
        });
    }

    fn capture_fps(&mut self, dt_sum: &mut f64, frame_sum: &mut f64, time: &mut Instant) {
        let now = Instant::now();
        *frame_sum += 1.0;
        self.dt = now.duration_since(*time).as_millis() as f64;
        *dt_sum += self.dt;
        if *dt_sum > 1000.0 {
            self.fps = 1000.0 / (*dt_sum as f64 / *frame_sum as f64);
            *dt_sum = 0.;
            *frame_sum = 0.;
        }
        *time = now;
    }
}
