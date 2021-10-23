use geng::draw_2d::TexturedVertex;
use geng::prelude::*;
use geng::Camera2d;

use generation::*;

mod biome;
mod init;

use biome::*;

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
    texture: Option<ugli::Texture>,
}

impl GenerationState {
    fn redraw_texture(&mut self) {
        let mut texture = ugli::Texture::new_with(
            self.geng.ugli(),
            self.framebuffer_size.map(|x| x as usize),
            |_| Color::BLACK,
        );

        let camera_view = camera_view(&self.camera, self.framebuffer_size);

        let mut temp_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut texture),
        );

        for (position, biome) in self.generator.view(aabb_to_area(camera_view)).tiles() {
            self.geng.draw_2d().quad(
                &mut temp_framebuffer,
                &self.camera,
                area_to_aabb(position),
                biome.color(),
            );
        }
        self.texture = Some(texture);
    }
}

impl geng::State for GenerationState {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);

        let camera_view = camera_view(&self.camera, self.framebuffer_size);

        if self.texture.is_none() {
            self.redraw_texture();
        }

        self.geng.draw_2d().textured(
            framebuffer,
            &self.camera,
            &[
                TexturedVertex {
                    a_pos: camera_view.bottom_left(),
                    a_color: Color::WHITE,
                    a_vt: vec2(0.0, 0.0),
                },
                TexturedVertex {
                    a_pos: camera_view.top_left(),
                    a_color: Color::WHITE,
                    a_vt: vec2(0.0, 1.0),
                },
                TexturedVertex {
                    a_pos: camera_view.top_right(),
                    a_color: Color::WHITE,
                    a_vt: vec2(1.0, 1.0),
                },
                TexturedVertex {
                    a_pos: camera_view.bottom_right(),
                    a_color: Color::WHITE,
                    a_vt: vec2(1.0, 0.0),
                },
            ],
            self.texture.as_ref().unwrap(),
            Color::WHITE,
            ugli::DrawMode::TriangleFan,
        );
    }
}

fn camera_view(camera: &Camera2d, framebuffer_size: Vec2<f32>) -> AABB<f32> {
    let vertical_fov = camera.fov;
    let horizontal_fov = framebuffer_size.x * vertical_fov / framebuffer_size.y;
    AABB::ZERO.extend_symmetric(vec2(horizontal_fov, vertical_fov) / 2.0)
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
