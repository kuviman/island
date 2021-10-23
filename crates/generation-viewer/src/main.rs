use geng::draw_2d::TexturedVertex;
use geng::prelude::*;
use geng::Camera2d;

use generation::*;

mod biome;
mod init;

use biome::*;

const CAMERA_ZOOM_SPEED: f32 = 0.5;
const CAMERA_FOV_MIN: f32 = 10.0;
const CAMERA_FOV_MAX: f32 = 500.0;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Generation Preview");
    let state = GenerationState::new(&geng);

    geng::run(&geng, state);
}

struct GenerationState {
    geng: Geng,
    camera: Camera2d,
    framebuffer_size: Vec2<f32>,
    generator: WorldGenerator<Biome>,
    textures: Vec<(Vec2<i32>, ugli::Texture)>,
    dragging: Option<Dragging>,
}

impl GenerationState {
    fn generate_view(&mut self) {
        let chunk_size = self.generator.chunk_size();
        let chunk_size = vec2(chunk_size.x, chunk_size.y);

        let camera_view = camera_view(&self.camera, self.framebuffer_size);
        let view = self.generator.generate_area(aabb_to_area(camera_view));

        self.textures.clear();

        for (chunk_pos, chunk) in view.chunks() {
            let mut texture =
                ugli::Texture::new_with(self.geng.ugli(), chunk_size, |_| Color::WHITE);

            let mut temp_framebuffer = ugli::Framebuffer::new_color(
                self.geng.ugli(),
                ugli::ColorAttachment::Texture(&mut texture),
            );

            for (position, biome) in chunk {
                let position = vec2(position.x as f32, position.y as f32);
                self.geng.draw_2d().quad(
                    &mut temp_framebuffer,
                    &geng::PixelPerfectCamera,
                    AABB::point(position).extend_positive(vec2(1.0, 1.0)),
                    biome.color(),
                );
            }
            self.textures
                .push((vec2(chunk_pos.x, chunk_pos.y), texture));
        }
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
                self.camera.fov -= delta as f32 * CAMERA_ZOOM_SPEED;
                self.camera.fov = self.camera.fov.clamp(CAMERA_FOV_MIN, CAMERA_FOV_MAX);
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
                    initial_camera: self.camera.center,
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
                                .camera
                                .screen_to_world(self.framebuffer_size, position.map(|x| x as f32));
                            let initial = self.camera.screen_to_world(
                                self.framebuffer_size,
                                initial_position.map(|x| x as f32),
                            );
                            let delta = initial - position;
                            self.camera.center = *initial_camera + delta;
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

        let chunk_size = self.generator.chunk_size().map(|x| x as f32) * self.generator.tile_size();
        let chunk_size = vec2(chunk_size.x, chunk_size.y);
        for (chunk_pos, texture) in &self.textures {
            let offset = chunk_pos.map(|x| x as f32) * chunk_size;
            self.geng.draw_2d().textured(
                framebuffer,
                &self.camera,
                &[
                    TexturedVertex {
                        a_pos: offset,
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 1.0),
                    },
                    TexturedVertex {
                        a_pos: offset + vec2(0.0, chunk_size.y),
                        a_color: Color::WHITE,
                        a_vt: vec2(0.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: offset + vec2(chunk_size.x, chunk_size.y),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 0.0),
                    },
                    TexturedVertex {
                        a_pos: offset + vec2(chunk_size.x, 0.0),
                        a_color: Color::WHITE,
                        a_vt: vec2(1.0, 1.0),
                    },
                ],
                texture,
                Color::WHITE,
                ugli::DrawMode::TriangleFan,
            );
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
