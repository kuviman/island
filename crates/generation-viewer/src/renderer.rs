use super::*;

pub struct Renderer {
    pub camera: Camera2d,
    geng: Geng,
    textures: Vec<(Vec2<i32>, ugli::Texture)>,
    chunk_size: Vec2<f32>,
    tile_size: Vec2<f32>,
}

impl Renderer {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            textures: Vec::new(),
            camera: Camera2d {
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 200.0,
            },
            chunk_size: vec2(1.0, 1.0),
            tile_size: vec2(1.0, 1.0),
        }
    }

    pub fn update_textures(&mut self, view: GenerationView<Biome>) {
        let chunk_size = vec2(view.chunk_size.x, view.chunk_size.y);
        self.chunk_size = chunk_size.map(|x| x as f32);
        self.tile_size = vec2(view.tile_size.x, view.tile_size.y);

        self.textures.clear();

        for (chunk_pos, chunk) in view.chunks() {
            let mut texture =
                ugli::Texture::new_with(self.geng.ugli(), chunk_size, |_| Color::WHITE);
            texture.set_filter(ugli::Filter::Nearest);

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

    pub fn draw(&self, framebuffer: &mut ugli::Framebuffer) {
        let chunk_size = self.chunk_size * self.tile_size;
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
