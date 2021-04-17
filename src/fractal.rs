use ggez::nalgebra as na;
use rug::{Complex, Float};

type Vector2 = na::Vector2<f32>;
type Point2 = na::Point2<f32>;

const PRECISION: u32 = 32;
const ITERATIONS: u32 = 160;

pub enum Fractal {
    Mandelbrot,
    BurningShip,
}

// track convergence endpoint, magnitude, and iteration
pub enum FractalBehavior {
    Converges(Complex, Float, u32),
    Diverges(Complex, Float, u32),
}

impl Fractal {
    pub fn iterate(&self, c: &Complex) -> FractalBehavior {
        let (z, mag, iter) = self.iterate_n(c, ITERATIONS);
        if mag > 2. {
            FractalBehavior::Diverges(z, mag, iter)
        } else {
            FractalBehavior::Converges(z, mag, iter)
        }
    }
    pub fn iterate_pt(&self, c: &Complex, ref_orbit: &Complex) -> FractalBehavior {
        let (z, mag, iter) = self.iterate_pn(ref_orbit, c, ITERATIONS);
        if mag > 2. {
            FractalBehavior::Diverges(z, mag, iter)
        } else {
            FractalBehavior::Converges(z, mag, iter)
        }
    }

    fn iterate_n(&self, c: &Complex, iter: u32) -> (Complex, Float, u32) {
        // z0 = 0
        let mut z: Complex = Complex::new(PRECISION);
        let mut mag: Float = Float::new(PRECISION);
        for i in 0..iter {
            z = match self {
                Fractal::Mandelbrot => {
                    // z1 = z^2 + c
                    z.square() + c
                }
                Fractal::BurningShip => {
                    // z1 = (re(z)+i*im(z))^2 + c
                    Complex::with_val(PRECISION, (z.real().clone().abs(), z.imag().clone().abs()))
                        .square()
                        + c
                }
            };
            mag = z.clone().norm().real().clone();
            if mag > 2 {
                return (z, mag, i);
            }
        }
        (z, mag, iter)
    }

    fn iterate_pn(&self, hp_z: &Complex, c: &Complex, iter: u32) -> (Complex, Float, u32) {
        let mut z: Complex = Complex::new(PRECISION);
        let mut mag: Float = Float::new(PRECISION);
        for i in 0..iter {
            z = match self {
                Fractal::Mandelbrot => {
                    // z1 = z^2 + c
                    hp_z * z.clone() * 2. + &z.square() + c
                }
                Fractal::BurningShip => {
                    // z1 = (re(z)+i*im(z))^2 + c
                    Complex::with_val(PRECISION, (z.real().clone().abs(), z.imag().clone().abs()))
                        .square()
                        + c
                }
            };
            mag = z.clone().norm().real().clone();
            if mag > 2 {
                return (z, mag, i);
            }
        }
        (z, mag, iter)
    }
}
