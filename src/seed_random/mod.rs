mod arc4;

use self::arc4::Arc4;

pub struct SeedRandom(Arc4);

const WIDTH: usize = 256;
const CHUNKS: usize = 6;

impl SeedRandom {
    pub fn new<I>(seed: I) -> Self
        where I: IntoIterator<Item = u8>
    {
        let seed = seed.into_iter().map(|b| b as f64).collect::<Vec<_>>();

        // Use the seed to initialize an ARC4 generator.
        SeedRandom(Arc4::new(&seed, WIDTH))
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
