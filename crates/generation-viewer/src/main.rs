use geng::draw_2d::TexturedVertex;
use geng::prelude::*;
use geng::Camera2d;

use generation::*;

mod biome;
mod init;
mod renderer;

use biome::*;
use renderer::*;

const CAMERA_ZOOM_SPEED: f32 = 0.5;
const CAMERA_FOV_MIN: f32 = 10.0;
const CAMERA_FOV_MAX: f32 = 1000.0;

const TILE_SIZE_MIN: f32 = 0.25;
const TILE_SIZE_MAX: f32 = 10.0;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Generation Preview");
    let state = GenerationState::new(&geng);

    geng::run(&geng, state);
}

struct GenerationState {
    geng: Geng,
    ui_camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    generator: WorldGenerator<Biome>,
    renderer: Renderer,
    dragging: Option<Dragging>,
}

impl GenerationState {
    fn generate_view(&mut self) {
        let camera_view = camera_view(&self.renderer.camera, self.framebuffer_size);
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
        match event {
            geng::Event::Wheel { delta } => {
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                    let mut new_scale = if delta > 0.0 {
                        self.generator.tile_size() * 2.0
                    } else {
                        self.generator.tile_size() / 2.0
                    };

                    // Check that the scale is not too small or too big
                    let min = TILE_SIZE_MIN;
                    let max = TILE_SIZE_MAX;
                    new_scale.x = new_scale.x.clamp(min, max);
                    new_scale.y = new_scale.y.clamp(min, max);
                    self.generator.set_scale(GenerationScale::TileSize {
                        x: new_scale.x,
                        y: new_scale.y,
                    });
                } else {
                    self.renderer.camera.fov -= delta as f32 * CAMERA_ZOOM_SPEED;
                    self.renderer.camera.fov = self
                        .renderer
                        .camera
                        .fov
                        .clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
                }
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
        let camera_view = camera_view(&self.ui_camera, self.framebuffer_size);

        self.geng.draw_2d().quad(
            framebuffer,
            &self.ui_camera,
            AABB::point(camera_view.bottom_right())
                .extend_left(30.0)
                .extend_up(camera_view.height()),
            Color::rgba(0.0, 0.0, 0.0, 0.5),
        );

        let tile_size = self.generator.tile_size() / self.renderer.camera.fov * self.ui_camera.fov;
        self.geng.draw_2d().quad(
            framebuffer,
            &self.ui_camera,
            AABB::point(camera_view.top_right() - vec2(10.0, 10.0))
                .extend_symmetric(vec2(tile_size.x, tile_size.y) / 2.0),
            Color::WHITE,
        );
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
