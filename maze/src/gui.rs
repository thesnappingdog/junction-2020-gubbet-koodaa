use crate::custom_events::CustomEvent;
use crate::game::MazeGame;
use imgui::{im_str, Condition, Context, FontSource, MenuItem, MouseCursor, Window as ImguiWindow};
use imgui_wgpu::{Renderer, RendererConfig, RendererResult};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use pixels::raw_window_handle::HasRawWindowHandle;
use pixels::wgpu::{
    CommandEncoder, LoadOp, Operations, RenderPassColorAttachmentDescriptor, RenderPassDescriptor,
    TextureFormat, TextureView,
};
use pixels::{Pixels, PixelsContext};
use raqote::Color;
use std::time::Instant;
use winit::error::ExternalError;
use winit::event::Event;
use winit::window::Window;

pub struct Gui {
    imgui: Context,
    platform: WinitPlatform,
    renderer: Renderer,
    dt_sum: f32,
    fps_str: String,
    last_frame: Instant,
    last_cursor: Option<MouseCursor>,
    metrics_open: bool,
    end_game_open: bool,
    restart: bool,
    winner: String,
    players: Vec<(String, Color)>,
}

impl Gui {
    pub fn new<W: HasRawWindowHandle>(window: &Window, pixels: &Pixels<W>) -> Self {
        let mut imgui = Context::create();
        imgui.set_ini_filename(None);
        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);
        let style = imgui.style_mut();
        for color in 0..style.colors.len() {
            style.colors[color] = gamma_to_linear(style.colors[color]);
        }
        let device = pixels.device();
        let queue = pixels.queue();
        let renderer = Renderer::new(
            &mut imgui,
            &device,
            &queue,
            RendererConfig::new().set_texture_format(TextureFormat::Bgra8UnormSrgb),
        );
        Gui {
            imgui,
            platform,
            renderer,
            dt_sum: 1000.0,
            fps_str: "".to_string(),
            last_frame: Instant::now(),
            last_cursor: None,
            metrics_open: false,
            end_game_open: false,
            winner: "".to_string(),
            restart: false,
            players: vec![],
        }
    }

    pub fn prepare(&mut self, window: &Window) -> Result<(), ExternalError> {
        let io = self.imgui.io_mut();
        io.update_delta_time(Instant::now() - self.last_frame);
        self.last_frame = Instant::now();
        self.platform.prepare_frame(io, window)
    }

    pub fn render(
        &mut self,
        window: &Window,
        encoder: &mut CommandEncoder,
        render_target: &TextureView,
        context: &PixelsContext,
    ) -> RendererResult<()> {
        let fps = self.imgui.io().framerate;
        let dt = self.imgui.io().delta_time * 1000.0;
        self.dt_sum += dt;
        if self.dt_sum > 1000.0 && fps != 0. {
            self.fps_str = format!("fps: {:.2}, dt: {:.2}", fps, 1000. / fps);
            self.dt_sum = 0.0
        }
        let ui = self.imgui.frame();
        let mut metrics_open = false;
        ui.main_menu_bar(|| {
            metrics_open = MenuItem::new(im_str!("Metrics")).build(&ui);
        });
        if metrics_open {
            self.metrics_open = true;
        }
        if self.metrics_open {
            ui.show_metrics_window(&mut self.metrics_open);
        }
        if self.end_game_open {
            let winner = self.winner.clone();
            ImguiWindow::new(im_str!("Game Over!"))
                .movable(false)
                .resizable(false)
                .collapsible(false)
                .position(
                    [
                        window.inner_size().width as f32 / window.scale_factor() as f32 / 2.
                            - 300.0 / 2.0,
                        window.inner_size().height as f32 / window.scale_factor() as f32 / 2.0
                            - 300.0 / 2.0,
                    ],
                    Condition::FirstUseEver,
                )
                .size([300.0, 300.0], Condition::FirstUseEver)
                .opened(&mut self.end_game_open)
                .build(&ui, || {
                    ui.text_colored([0., 1.0, 0., 1.0], im_str!("Winner: {}", winner));
                });
            if !self.end_game_open {
                self.restart = true;
            }
        }
        let player_names = self.players.clone();
        ImguiWindow::new(im_str!("Players!"))
            .movable(false)
            .resizable(false)
            .collapsible(false)
            .position([0., 20.], Condition::FirstUseEver)
            .size([150.0, 300.0], Condition::FirstUseEver)
            .build(&ui, || {
                for p in player_names {
                    ui.text_colored(
                        [
                            p.1.r() as f32 / 255.0,
                            p.1.g() as f32 / 255.0,
                            p.1.b() as f32 / 255.0,
                            1.0,
                        ],
                        im_str!("{}", p.0),
                    );
                }
            });
        let mouse_cursor = ui.mouse_cursor();
        if self.last_cursor != mouse_cursor {
            self.last_cursor = mouse_cursor;
            self.platform.prepare_render(&ui, window);
        }

        let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: render_target,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        self.renderer
            .render(ui.render(), &context.queue, &context.device, &mut rpass)
    }

    pub fn handle_event(
        &mut self,
        window: &Window,
        event: &Event<CustomEvent>,
        game: &mut MazeGame,
    ) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
        if self.restart {
            game.restart();
            self.restart = false;
        }
        if let Some(winner) = game.winner_name() {
            self.end_game_open = true;
            self.winner = winner;
        }
        self.players = game.players();
    }
}

fn gamma_to_linear(color: [f32; 4]) -> [f32; 4] {
    const GAMMA: f32 = 2.2;
    let x = color[0].powf(GAMMA);
    let y = color[1].powf(GAMMA);
    let z = color[2].powf(GAMMA);
    let w = 1.0 - (1.0 - color[3]).powf(GAMMA);
    [x, y, z, w]
}
