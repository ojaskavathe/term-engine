use rand::prelude::*;

struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn dot(&self, other: Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}

pub fn perlin_noise(x: f64, y: f64) -> f64 {
    let xi = x.floor() as u32 & 255;
    let yi = y.floor() as u32 & 255;

    let xf = x - x.floor();
    let yf = y - y.floor();

    let top_right = Vec2 {
        x: xf - 1.0,
        y: yf - 1.0,
    };
    let top_left = Vec2 { x: xf, y: yf - 1.0 };
    let bottom_left = Vec2 { x: xf, y: yf };
    let bottom_right = Vec2 { x: xf - 1.0, y: yf };

    let mut p: [u32; 256] = std::array::from_fn(|i| i as u32);
    p.shuffle(&mut thread_rng());
    let mut permutation = [0u32; 512];
    permutation[..256].copy_from_slice(&p);
    permutation[256..].copy_from_slice(&p);

    let val_top_right = permutation[(permutation[(xi + 1) as usize] + yi + 1) as usize];
    let val_top_left = permutation[(permutation[xi as usize] + yi + 1) as usize];
    let val_bottom_left = permutation[(permutation[xi as usize] + yi) as usize];
    let val_bottom_right = permutation[(permutation[(xi + 1) as usize] + yi) as usize];

    fn constant(v: u32) -> Vec2 {
        let h = v & 3;
        match h {
            0 => Vec2 { x: 1.0, y: 1.0 },
            1 => Vec2 { x: 1.0, y: -1.0 },
            2 => Vec2 { x: -1.0, y: -1.0 },
            3 => Vec2 { x: -1.0, y: 1.0 },
            _ => Vec2 { x: 1.0, y: 1.0 },
        }
    }

    let dot_top_right = top_right.dot(constant(val_top_right));
    let dot_top_left = top_left.dot(constant(val_top_left));
    let dot_bottom_left = bottom_left.dot(constant(val_bottom_left));
    let dot_bottom_right = bottom_right.dot(constant(val_bottom_right));

    fn fade(t: f64) -> f64 {
        ((6.0 * t - 15.0) * t + 10.0) * t * t * t
    }

    let u = fade(xf);
    let v = fade(yf);

    fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
        a1 + t * (a2 - a1)
    }

    lerp(u,
        lerp(v, dot_bottom_left, dot_top_left),
        lerp(v, dot_bottom_right, dot_top_right),
    )
}
