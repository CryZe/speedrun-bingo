use super::low_bits;

#[derive(Debug)]
pub struct Arc4 {
    i: f64,
    j: f64,
    width: usize,
    s: Vec<f64>,
}

impl Arc4 {
    /// An ARC4 implementation. The constructor takes a key in the form of
    /// an array of at most (width) integers that should be 0 <= x < (width).
    pub fn new(mut key: &[f64], width: usize) -> Self {
        let mut s = Vec::new();
        let mut j = 0.0;

        // The empty key [] is treated as [0].
        if key.is_empty() {
            static EMPTY: &'static [f64] = &[0.0];
            key = EMPTY;
        }

        // Set up S using the standard key scheduling algorithm.
        s.extend((0..width).map(|i| i as f64));
        for i in 0..width {
            let i = i as f64;
            let t = s[i as usize];
            j = low_bits(j + t + key[i as usize % key.len()], width);
            let u = s[j as usize];
            s[i as usize] = u;
            s[j as usize] = t;
        }

        let mut me = Arc4 {
            i: 0.0,
            j: 0.0,
            width: width,
            s: s,
        };

        // For robust unpredictability discard an initial batch of values.
        // See http://www.rsa.com/rsalabs/node.asp?id=2009
        me.generate(width);

        me
    }

    /// The generate(count) method returns a pseudorandom integer that concatenates
    /// the next (count) outputs from ARC4.  Its return value is a number x
    /// that is in the range 0 <= x < (width ^ count).
    pub fn generate(&mut self, count: usize) -> f64 {
        let width = self.width;

        let mut i = low_bits(self.i + 1.0, width);
        let mut t = self.s[i as usize];
        let mut j = low_bits(self.j + t, width);
        let mut u = self.s[j as usize];
        self.s[i as usize] = u;
        self.s[j as usize] = t;
        let mut r = self.s[low_bits(t + u, width) as usize];

        for _ in 1..count {
            i = low_bits(i + 1.0, width);
            t = self.s[i as usize];
            j = low_bits(j + t, width);
            u = self.s[j as usize];
            self.s[i as usize] = u;
            self.s[j as usize] = t;
            r = r * width as f64 + self.s[low_bits(t + u, width) as usize];
        }

        self.i = i;
        self.j = j;

        r
    }
}

#[test]
fn test() {
    let arc4 = Arc4::new(&[57.0, 48.0, 49.0, 54.0, 50.0, 51.0], 256);
    assert_eq!(arc4.s,
               vec![250.0, 175.0, 217.0, 198.0, 168.0, 238.0, 130.0, 229.0, 71.0, 241.0, 225.0,
                    255.0, 206.0, 98.0, 13.0, 179.0, 74.0, 149.0, 143.0, 81.0, 139.0, 224.0,
                    62.0, 254.0, 107.0, 39.0, 97.0, 135.0, 210.0, 32.0, 22.0, 14.0, 248.0, 21.0,
                    239.0, 142.0, 104.0, 153.0, 63.0, 146.0, 126.0, 128.0, 54.0, 59.0, 35.0,
                    159.0, 243.0, 83.0, 144.0, 23.0, 44.0, 110.0, 174.0, 69.0, 29.0, 184.0,
                    176.0, 158.0, 47.0, 53.0, 90.0, 82.0, 77.0, 154.0, 228.0, 96.0, 150.0, 165.0,
                    191.0, 193.0, 246.0, 185.0, 131.0, 37.0, 132.0, 189.0, 18.0, 94.0, 127.0,
                    67.0, 252.0, 109.0, 207.0, 95.0, 43.0, 186.0, 78.0, 202.0, 45.0, 220.0,
                    115.0, 169.0, 218.0, 27.0, 232.0, 155.0, 88.0, 25.0, 61.0, 51.0, 24.0, 253.0,
                    1.0, 177.0, 162.0, 26.0, 10.0, 221.0, 65.0, 233.0, 197.0, 160.0, 211.0,
                    148.0, 19.0, 234.0, 157.0, 227.0, 70.0, 151.0, 214.0, 216.0, 15.0, 7.0,
                    201.0, 12.0, 108.0, 0.0, 73.0, 36.0, 145.0, 208.0, 124.0, 101.0, 34.0, 125.0,
                    163.0, 240.0, 68.0, 41.0, 230.0, 9.0, 87.0, 156.0, 209.0, 161.0, 56.0, 170.0,
                    212.0, 133.0, 84.0, 121.0, 48.0, 5.0, 66.0, 89.0, 192.0, 181.0, 93.0, 231.0,
                    190.0, 55.0, 100.0, 236.0, 178.0, 105.0, 122.0, 247.0, 242.0, 111.0, 3.0,
                    50.0, 30.0, 134.0, 245.0, 123.0, 113.0, 147.0, 152.0, 136.0, 180.0, 183.0,
                    85.0, 120.0, 164.0, 118.0, 52.0, 140.0, 119.0, 114.0, 141.0, 106.0, 86.0,
                    28.0, 219.0, 187.0, 172.0, 215.0, 188.0, 203.0, 200.0, 235.0, 244.0, 103.0,
                    38.0, 112.0, 33.0, 205.0, 75.0, 72.0, 204.0, 76.0, 92.0, 129.0, 116.0, 58.0,
                    226.0, 237.0, 173.0, 8.0, 60.0, 137.0, 42.0, 223.0, 40.0, 195.0, 31.0, 49.0,
                    167.0, 196.0, 194.0, 251.0, 2.0, 4.0, 11.0, 79.0, 171.0, 213.0, 182.0, 80.0,
                    102.0, 222.0, 64.0, 166.0, 117.0, 6.0, 57.0, 249.0, 16.0, 46.0, 199.0, 138.0,
                    99.0, 20.0, 91.0, 17.0]);
    assert_eq!(arc4.i, 0.0);
    assert_eq!(arc4.j, 139.0);
    assert_eq!(arc4.width, 256);
}
