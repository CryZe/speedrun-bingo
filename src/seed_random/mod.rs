use arrayvec::ArrayVec;

#[cfg(not(feature = "std"))]
trait Float {
    fn powi(&self, exp: usize) -> Self;
}

#[cfg(not(feature = "std"))]
impl Float for f64 {
    fn powi(&self, mut n: usize) -> Self {
        let mut x = *self;
        if n == 0 {
            return 1.0;
        }
        let mut y = 1.0;
        while n > 1 {
            if n & 1 != 0 {
                y *= x;
            }
            x *= x;
            n >>= 1;
        }
        x * y
    }
}

mod arc4;

use self::arc4::Arc4;

pub struct SeedRandom(Arc4);

const WIDTH: usize = 256;
const CHUNKS: usize = 6;

impl SeedRandom {
    pub fn new(seed: ArrayVec<[u8; 10]>) -> Self {
        let seed = seed.into_iter().map(|b| b as f64).collect();

        // Use the seed to initialize an ARC4 generator.
        SeedRandom(Arc4::new(seed, WIDTH))
    }

    pub fn next(&mut self) -> f64 {
        let start_denom = (WIDTH as f64).powi(CHUNKS as _);
        let significance = 2.0f64.powi(52);
        let overflow = 2.0 * significance;

        let mut n = self.0.generate(CHUNKS);
        let mut d = start_denom;
        let mut x = 0.0;
        while n < significance {
            n = (n + x) * WIDTH as f64;
            d *= WIDTH as f64;
            x = self.0.generate(1);
        }
        while n >= overflow {
            n /= 2.0;
            d /= 2.0;
            x = (((x as i64) as u64 >> 1) as i64) as f64;
        }
        (n + x) / d
    }
}

fn low_bits(n: f64, width: usize) -> f64 {
    (n as i64 & (width as i64 - 1)) as f64
}
