use arrayvec::{ArrayString, ArrayVec};
use core::fmt::Write;
use {Bingo, Goal, Mode, Template};
use seed_random::SeedRandom;

static LINE_CHECK_LIST: &[&[usize]; 25] = &[
    &[1usize, 2, 3, 4, 5, 10, 15, 20, 6, 12, 18, 24] as &[_],
    &[0, 2, 3, 4, 6, 11, 16, 21],
    &[0, 1, 3, 4, 7, 12, 17, 22],
    &[0, 1, 2, 4, 8, 13, 18, 23],
    &[0, 1, 2, 3, 8, 12, 16, 20, 9, 14, 19, 24],
    &[0, 10, 15, 20, 6, 7, 8, 9],
    &[0, 12, 18, 24, 5, 7, 8, 9, 1, 11, 16, 21],
    &[5, 6, 8, 9, 2, 12, 17, 22],
    &[4, 12, 16, 20, 9, 7, 6, 5, 3, 13, 18, 23],
    &[4, 14, 19, 24, 8, 7, 6, 5],
    &[0, 5, 15, 20, 11, 12, 13, 14],
    &[1, 6, 16, 21, 10, 12, 13, 14],
    &[0, 6, 12, 18, 24, 20, 16, 8, 4, 2, 7, 17, 22, 10, 11, 13, 14],
    &[3, 8, 18, 23, 10, 11, 12, 14],
    &[4, 9, 19, 24, 10, 11, 12, 13],
    &[0, 5, 10, 20, 16, 17, 18, 19],
    &[15, 17, 18, 19, 1, 6, 11, 21, 20, 12, 8, 4],
    &[15, 16, 18, 19, 2, 7, 12, 22],
    &[15, 16, 17, 19, 23, 13, 8, 3, 24, 12, 6, 0],
    &[4, 9, 14, 24, 15, 16, 17, 18],
    &[0, 5, 10, 15, 16, 12, 8, 4, 21, 22, 23, 24],
    &[20, 22, 23, 24, 1, 6, 11, 16],
    &[2, 7, 12, 17, 20, 21, 23, 24],
    &[20, 21, 22, 24, 3, 8, 13, 18],
    &[0, 6, 12, 18, 20, 21, 22, 23, 19, 14, 9, 4],
];

fn difficulty(seed: u32, i: usize, mode: Mode) -> i64 {
    // To create the magic square we need 2 random orderings of the numbers 0, 1, 2, 3, 4.
    // The following creates those orderings and calls them Table5 and Table1

    let num3 = seed % 1000; // Table5 will use the ones, tens, and hundreds digits.

    let rem8 = num3 % 8;
    let rem4 = rem8 / 2;
    let rem2 = rem8 % 2;
    let rem5 = num3 % 5;
    let rem3 = num3 % 3; // Note that Rem2, Rem3, Rem4, and Rem5 are mathematically independent.
    let rem_t = num3 / 120; // This is between 0 and 8

    // The idea is to begin with an array containing a single number, 0.
    // Each number 1 through 4 is added in a random spot in the array's current size.
    // The result - the numbers 0 to 4 are in the array in a random (and uniform) order.
    let mut table5 = ArrayVec::<[_; 5]>::new();
    table5.push(0);
    table5.insert(rem2 as usize, 1);
    table5.insert(rem3 as usize, 2);
    table5.insert(rem4 as usize, 3);
    table5.insert(rem5 as usize, 4);

    let num3 = seed / 1000; // Table1 will use the next 3 digits.
    let num3 = num3 % 1000;

    let rem8 = num3 % 8;
    let rem4 = rem8 / 2;
    let rem2 = rem8 % 2;
    let rem5 = num3 % 5;
    let rem3 = num3 % 3;
    let rem_t = rem_t * 8 + num3 / 120; // This is between 0 and 64.

    let mut table1 = ArrayVec::<[_; 5]>::new();
    table1.push(0);
    table1.insert(rem2 as usize, 1);
    table1.insert(rem3 as usize, 2);
    table1.insert(rem4 as usize, 3);
    table1.insert(rem5 as usize, 4);

    let i = i - 1;
    let rem_t = rem_t % 5; // Between 0 and 4, fairly uniformly.
    let x = (i + rem_t as usize) % 5; // RemT is horizontal shift to put any diagonal on the main diagonal.
    let y = i / 5;

    // The Tables are set into a single magic square template
    // Some are the same up to some rotation, reflection, or row permutation.
    // However, all genuinely different magic squares can arise in this fashion.
    let e5 = table5[((x + 3 * y) % 5) as usize];
    let e1 = table1[((3 * x + y) % 5) as usize];

    // Table5 controls the 5* part and Table1 controls the 1* part.
    let value = 5 * e5 + e1;

    match mode {
        Mode::Short => value / 2, // if short mode, limit difficulty
        Mode::Long | Mode::Special => (value + 25) / 2,
        _ => value,
    }
}

fn check_line<'a, I, U>(i: usize, types_a: &'a I, gen_cells: &[GenCell]) -> usize
where
    &'a I: IntoIterator<Item = U>,
    U: AsRef<str>,
{
    let mut synergy = 0;

    for j in 0..LINE_CHECK_LIST[i].len() {
        if let Some(types_b) = gen_cells.get(LINE_CHECK_LIST[i][j]) {
            let types_b = &types_b.goal.types;
            for (k, tk) in types_a.into_iter().enumerate() {
                let tk = tk.as_ref();
                for (l, tl) in types_b.iter().enumerate() {
                    if tk == tl {
                        synergy += 1; // if match increase
                        if k == 0 {
                            synergy += 1; // if main type increase
                        }
                        if l == 0 {
                            synergy += 1; // if main type increase
                        }
                    }
                }
            }
        }
    }

    synergy
}

struct GenCell<'a> {
    goal: &'a Goal,
    synergy: usize,
}

pub fn generate<'a>(seed: u32, mode: Mode, template: &'a Template) -> Bingo<'a> {
    let mut seed_str = ArrayString::<[_; 10]>::new();
    write!(seed_str, "{}", seed).unwrap();
    let mut random = SeedRandom::new(seed_str.as_bytes().iter().cloned().collect());

    let mut cells = <[[&str; 5]; 5]>::default();

    // populate the bingo board in the array
    let mut gen_cells = ArrayVec::<[_; 25]>::new();
    for i in 1..26 {
        let difficulty = difficulty(seed, i, mode); // difficulty of current square
        let template_part = &template.0[difficulty as usize];
        let rng = (template_part.len() as f64 * random.next()) as usize;
        let mut j = 0;
        let mut gen_cell = None::<GenCell>;
        loop {
            let current_obj = &template_part[(j + rng) % template_part.len()];
            let synergy = check_line(i - 1, &current_obj.types, &gen_cells);
            if gen_cell.as_ref().map_or(true, |c| synergy < c.synergy) {
                gen_cell = Some(GenCell {
                    goal: current_obj,
                    synergy: synergy,
                });
            }

            j += 1;
            if synergy == 0 || j >= template_part.len() {
                break;
            }
        }
        gen_cells.push(gen_cell.unwrap());
    }

    // populate the actual table
    for (c, g) in cells
        .iter_mut()
        .flat_map(|r| r.iter_mut())
        .zip(gen_cells.into_iter())
    {
        *c = &g.goal.name;
    }

    Bingo { cells: cells }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Template;

    #[test]
    fn test() {
        let sm64 = include_str!("templates/sm64.json");
        let template = Template::from_json_str(sm64).unwrap();
        let bingo = template.generate(587062, Mode::Normal);
        assert_eq!(
            bingo,
            Bingo {
                cells: [
                    [
                        "All Stars in TTM".into(),
                        "2 Cap Stage Stars".into(),
                        "100 Coin Star in CCM".into(),
                        "6 Stars in DDD".into(),
                        "6 Stars in HMC".into()
                    ],
                    [
                        "6 Stars in RR".into(),
                        "100 Coin Star in BBH".into(),
                        "6 Stars in TTM".into(),
                        "100 Coin Star in SSL".into(),
                        "Peach\'s Slide x 2".into()
                    ],
                    [
                        "One Star in All Even Number Courses".into(),
                        "Cruiser Crossing the Rainbow RR".into(),
                        "At least 1 Star from each Stage".into(),
                        "All Stars in LLL".into(),
                        "Open 9 Cannons".into()
                    ],
                    [
                        "Top Floor Cloud Stage Star".into(),
                        "5 Stars in DDD".into(),
                        "Three Bowser Stage Red Coin Stars".into(),
                        "Open 3 cannons".into(),
                        "All Stars in RR".into()
                    ],
                    [
                        "Three 100 Coin Stars".into(),
                        "All Stars in THI".into(),
                        "3 Stars each from JRB and BBH".into(),
                        "At least 3 stars from 6 stages".into(),
                        "100 Coin Star in LLL".into()
                    ]
                ],
            }
        );
    }
}
