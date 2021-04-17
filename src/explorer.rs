use ggez::event::EventHandler;
use ggez::graphics::{self, DrawParam, Image};
use ggez::timer::{self};
use ggez::{Context, GameResult};

use ggez::nalgebra as na;
use rug::{Complex, Float};

use crate::fractal::{Fractal, FractalBehavior};

type Vector2 = na::Vector2<f32>;
type Point2 = na::Point2<f32>;

const PRECISION: u32 = 16;
const DEFAULT_ZOOM: f32 = 0.5;
const DEFAULT_POSITION: (f32, f32) = (-0.756, 0.);
const DISPLAY_RES: (usize, usize) = (1000, 750);
const RENDER_RES: (usize, usize) = (200, 150);
const ITERATIONS: u32 = 160;

const RENDER_BUFFER_SIZE: usize = RENDER_RES.0 * RENDER_RES.1 * 4;

pub struct Explorer {
    zoom: Float,
    position: Complex,
    fractal: Fractal,
    buffer: [u8; RENDER_BUFFER_SIZE],
    update_screen: bool,
}

impl Explorer {
    pub fn new(_ctx: &mut Context) -> Explorer {
        Explorer {
            zoom: Float::with_val(PRECISION, DEFAULT_ZOOM),
            position: Complex::with_val(PRECISION, DEFAULT_POSITION),
            buffer: [0; RENDER_BUFFER_SIZE],
            fractal: Fractal::Mandelbrot,
            update_screen: true,
        }
    }

    pub fn world_to_complex(&self, x: usize, y: usize) -> Complex {
        let dx = (((x as f32) / RENDER_RES.0 as f32) - 0.5) * 2.;
        let dy = (((y as f32) / RENDER_RES.1 as f32) - 0.5) * 2.;
        let cmpx = Complex::with_val(PRECISION, (dx, dy));
        &self.zoom * cmpx + &self.position
    }

    pub fn fill_image_buffer_pt(&mut self) -> GameResult<()> {
        let width = RENDER_RES.0;
        let height = RENDER_RES.1;
        let blk_size = 100 * 100;
        for blk_idx in 0..height * width / blk_size {
            // calculate tile-based reference orbit
            let ref_idx = blk_idx * blk_size;
            let ref_x = ref_idx % width;
            let ref_y = ref_idx / width;
            let ref_coords = self.world_to_complex(ref_x, ref_y);
            let ref_result = self.fractal.iterate(&ref_coords);
            // use pertubation for the rest of the points
            for blk_offset in 0..blk_size {
                let idx = blk_idx * blk_size + blk_offset;
                // compute fractal result
                let x = idx % width;
                let y = idx / width;
                let coords = self.world_to_complex(x, y);
                let behavior;
                // use reference orbit if valid
                if let FractalBehavior::Converges(ref_orbit, _, _) = &ref_result {
                    behavior = self.fractal.iterate_pt(&coords, &ref_orbit);
                } else {
                    behavior = self.fractal.iterate(&coords);
                }
                // color result depending on behavior
                let rgb: [u8; 3] = match behavior {
                    FractalBehavior::Converges(_, _, _) => [0; 3],
                    FractalBehavior::Diverges(_, _, c) => {
                        let cf32 = c as f32;
                        let if32 = ITERATIONS as f32;
                        let color = (cf32 / if32 * 256.) as u8;
                        [color; 3]
                    }
                };
                // write to image buffer
                self.buffer[idx * 4] = rgb[0];
                self.buffer[idx * 4 + 1] = rgb[1];
                self.buffer[idx * 4 + 2] = rgb[2];
                self.buffer[idx * 4 + 3] = 255;
            }
        }
        Ok(())
    }

    pub fn fill_image_buffer(&mut self) -> GameResult<()> {
        let width = RENDER_RES.0;
        let height = RENDER_RES.1;
        for idx in 0..height * width {
            // compute fractal result
            let x = idx % width;
            let y = idx / width;
            let coords = self.world_to_complex(x, y);
            let behavior = self.fractal.iterate(&coords);
            // color result depending on behavior
            let rgb: [u8; 3] = match behavior {
                FractalBehavior::Converges(_, _, _) => [0; 3],
                FractalBehavior::Diverges(_, _, c) => {
                    let cf32 = c as f32;
                    let if32 = ITERATIONS as f32;
                    let color = (cf32 / if32 * 256.) as u8;
                    [color; 3]
                }
            };
            // write to image buffer
            self.buffer[idx * 4] = rgb[0];
            self.buffer[idx * 4 + 1] = rgb[1];
            self.buffer[idx * 4 + 2] = rgb[2];
            self.buffer[idx * 4 + 3] = 255;
        }
        Ok(())
    }
}

impl EventHandler for Explorer {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        // limit to 60 fps
        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.update_screen {
                self.fill_image_buffer()?;
                self.update_screen = false;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let img_scale_x = (DISPLAY_RES.0 as f32) / (RENDER_RES.0 as f32);
        let img_scale_y = (DISPLAY_RES.1 as f32) / (RENDER_RES.1 as f32);
        let img_params = DrawParam::new()
            .dest(Point2::new(0.0, 0.0))
            .scale(Vector2::new(img_scale_x, img_scale_y));

        graphics::clear(ctx, graphics::WHITE);

        let img_x = RENDER_RES.0 as u16;
        let img_y = RENDER_RES.1 as u16;
        let img = Image::from_rgba8(ctx, img_x, img_y, &self.buffer).unwrap();
        graphics::draw(ctx, &img, img_params)?;

        graphics::present(ctx)
    }
}
