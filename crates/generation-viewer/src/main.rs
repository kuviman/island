use geng::draw_2d::TexturedVertex;
use geng::prelude::*;
use geng::Camera2d;

use generation::*;

mod biome;
mod config;
mod init;
mod interface;
mod renderer;

use biome::*;
use config::*;
use interface::*;
use renderer::*;

const CAMERA_ZOOM_SPEED: f32 = 0.5;
const CAMERA_FOV_MIN: f32 = 10.0;
const CAMERA_FOV_MAX: f32 = 1000.0;

const RESOLUTION_MIN: u32 = 64;
const RESOLUTION_MAX: u32 = 256;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }

    let geng = Geng::new("Generation Preview");
    let state = GenerationState::new(&geng);

    geng::run(&geng, state);
}

struct GenerationState {
    framebuffer_size: Vec2<f32>,
    generator: WorldGenerator<Biome>,
    dragging: Option<Dragging>,
    renderer: Renderer,
    ui_state: UIState,
    ui_controller: geng::ui::Controller,
}

impl GenerationState {
    fn generate_view(&mut self) {
        let camera_view = camera_view(&self.renderer.camera, self.framebuffer_size);

        let tile_size = camera_view.width() / self.ui_state.resolution() as f32;
        self.generator.set_scale(GenerationScale::TileSize {
            x: tile_size,
            y: tile_size,
        });

        let view = self.generator.generate_area(aabb_to_area(camera_view));

        self.renderer.update_textures(view);
    }
}

enum Dragging {
    Move {
        initial_mouse: Vec2<f32>,
        initial_camera: Vec2<f32>,
    },
}

impl geng::State for GenerationState {
    fn handle_event(&mut self, event: geng::Event) {
        if self
            .ui_controller
            .handle_event(&mut self.ui_state.ui(), event.clone())
        {
            return;
        }

        match event {
            geng::Event::Wheel { delta } => {
                self.renderer.camera.fov -= delta as f32 * CAMERA_ZOOM_SPEED;
                self.renderer.camera.fov = self
                    .renderer
                    .camera
                    .fov
                    .clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
            }
            geng::Event::KeyDown {
                key: geng::Key::Space,
            } => {
                self.generate_view();
            }
            geng::Event::MouseDown {
                button: geng::MouseButton::Left,
                position,
            } => {
                self.dragging = Some(Dragging::Move {
                    initial_mouse: position.map(|x| x as f32),
                    initial_camera: self.renderer.camera.center,
                });
            }
            geng::Event::MouseUp {
                button: geng::MouseButton::Left,
                ..
            } => {
                self.dragging = None;
            }
            geng::Event::MouseMove { position, .. } => {
                if let Some(dragging) = &self.dragging {
                    match dragging {
                        Dragging::Move {
                            initial_mouse: initial_position,
                            initial_camera,
                        } => {
                            let position = self
                                .renderer
                                .camera
                                .screen_to_world(self.framebuffer_size, position.map(|x| x as f32));
                            let initial = self.renderer.camera.screen_to_world(
                                self.framebuffer_size,
                                initial_position.map(|x| x as f32),
                            );
                            let delta = initial - position;
                            self.renderer.camera.center = *initial_camera + delta;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.renderer.draw(framebuffer);

        // UI
        self.ui_controller
            .draw(&mut self.ui_state.ui(), framebuffer);
        // let tile_size = self.generator.tile_size();
        // self.ui_state.draw(
        //     framebuffer,
        //     vec2(tile_size.x, tile_size.y),
        //     self.renderer.camera.fov,
        // );
    }

    fn update(&mut self, delta_time: f64) {
        self.ui_controller
            .update(&mut self.ui_state.ui(), delta_time);

        let mut generate = false;
        for event in self.ui_state.events() {
            match event {
                UIEvent::Generate => {
                    generate = true;
                }
            }
        }

        if generate {
            self.generate_view();
        }
    }
}

fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    let vertical_fov = camera.fov;
    let horizontal_fov = framebuffer_size.x * vertical_fov / framebuffer_size.y;
    AABB::ZERO
        .extend_symmetric(vec2(horizontal_fov, vertical_fov) / 2.0)
        .translate(camera.center)
}

fn aabb_to_area<T>(aabb: AABB<T>) -> Area<T> {
    Area {
        start: Vector2::new(aabb.x_min, aabb.y_min),
        end: Vector2::new(aabb.x_max, aabb.y_max),
    }
}

fn area_to_aabb<T>(area: Area<T>) -> AABB<T> {
    AABB {
        x_min: area.start.x,
        x_max: area.end.x,
        y_min: area.start.y,
        y_max: area.end.y,
    }
}
