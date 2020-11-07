use crate::custom_events::{handle_client, CustomEvent};
use crate::game::MazeGame;
use crate::gui::Gui;
use crate::window::AppWindow;
use log::error;
use pixels::Error;
use std::net::TcpListener;
use std::time::Instant;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

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
        // Listen player connections & incoming events, handle client creates custom events based on those
        std::thread::spawn(move || loop {
            for stream in listener.incoming() {
                handle_client(stream.expect("Failed to read stream"), &event_loop_proxy)
                    .expect("Failed to handle tcp message");
            }
        });
        event_loop.run(move |event, _, control_flow| {
            gui.handle_event(&window.window(), &event, &mut game);
            if let Event::RedrawRequested(_) = event {
                window.clear().expect("Failed to clear");
                // Update game (render etc...) after redraw request
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
            // Custom events must be handled before input.update, since input.update consumes events
            // Though currently input ain't used...
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
