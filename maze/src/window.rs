use crate::app::CustomEvent;
use crate::gui::Gui;
use pixels::{Error, Pixels, PixelsBuilder, SurfaceTexture};
use rayon::prelude::*;
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct AppWindow {
    window: Window,
    framebuffer: Pixels<Window>,
    width: u32,
    height: u32,
}

impl AppWindow {
    pub fn new(
        title: &str,
        event_loop: &EventLoop<CustomEvent>,
        win_width: u32,
        win_height: u32,
    ) -> AppWindow {
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_visible(false)
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let hidpi_factor = window.scale_factor();
        let (monitor_width, monitor_height) = {
            let size = window.current_monitor().size();
            (
                size.width as f64 / hidpi_factor,
                size.height as f64 / hidpi_factor,
            )
        };
        let default_size = LogicalSize::new(win_width, win_height);
        let center = LogicalPosition::new(
            (monitor_width - win_width as f64) / 2.0,
            (monitor_height - win_height as f64) / 2.0,
        );
        window.set_inner_size(default_size);
        window.set_min_inner_size(Some(default_size));
        window.set_outer_position(center);
        window.set_visible(true);
        let framebuffer = {
            let surface_texture = SurfaceTexture::new(win_width, win_height, &window);
            PixelsBuilder::new(win_width, win_height, surface_texture)
                .enable_vsync(true)
                .build()
                .expect("Failed to create framebuffer")
        };
        AppWindow {
            window,
            framebuffer,
            width: win_width,
            height: win_height,
        }
    }

    #[allow(dead_code)]
    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.framebuffer.resize(new_size.width, new_size.height);
    }

    #[allow(dead_code)]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    #[allow(dead_code)]
    pub fn framebuffer(&mut self) -> &mut [u8] {
        self.framebuffer.get_frame()
    }

    pub fn pixels(&self) -> &Pixels<Window> {
        &self.framebuffer
    }

    #[allow(dead_code)]
    pub fn present(&mut self) -> Result<(), Error> {
        self.framebuffer.render()
    }

    pub fn present_with_gui(&mut self, gui: &mut Gui) -> Result<(), Error> {
        gui.prepare(&self.window).expect("gui.prepare() failed");
        let window = &self.window;
        self.framebuffer
            .render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);
                gui.render(window, encoder, render_target, context)
                    .expect("gui.render() failed");
            })
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.framebuffer.get_frame().par_iter_mut().for_each(|p| {
            *p = 255;
        });
        Ok(())
    }

    pub fn request_redraw(&mut self) {
        self.window.request_redraw();
    }
}
