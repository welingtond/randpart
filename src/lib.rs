// RandPart is a structure made up from randomic ascii characters, whose size and presence of
// special characters are defined by the arguments in the [`new`] method's calling.
//! # RandPart
//!
//! A simple structure representing aleatory non-repetible ascii characters.
//! To use in situations like when generating passwords.
#![warn(missing_docs)]
use rand::Rng;

/// An array of non-capital letters.
const LMIN: [u8; 26] = [
    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122,
];

/// An array of capital letters.
const LMAI: [u8; 26] = [
    65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90,
];

/// An array of numbers.
const NUMB: [u8; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];

/// An array of special characters.
const ESPC: [u8; 32] = [
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91, 92,
    93, 94, 95, 96, 123, 124, 125, 126,
];

/// The main structure RandPart, which is an u8's vector.
#[derive(Debug)]
pub struct RandPart {
    rand_portion: Vec<u8>,
}

impl RandPart {
    /// Creates a RandPart, based on the arguments: size and use/don't use special characters.
    pub fn new(sz: usize, random: bool) -> RandPart {
        let mut res: Vec<u8> = vec![];
        let mut qlist: u8;
        let list_size: u8;

        match random {
            true => {
                list_size = 3;
            }
            false => {
                list_size = 2;
            }
        };

        for _x in 0..sz {
            let mut rng = rand::thread_rng();
            qlist = rng.gen_range(0..=list_size);
            //
            match qlist {
                0 => res.push(LMIN[rng.gen_range(0..LMIN.len())]),
                1 => res.push(LMAI[rng.gen_range(0..LMAI.len())]),
                2 => res.push(NUMB[rng.gen_range(0..NUMB.len())]),
                3 => res.push(ESPC[rng.gen_range(0..ESPC.len())]),
                _ => res.push(NUMB[rng.gen_range(0..NUMB.len())]),
                // ureachable, but if it makes the compiler happy, let it be.
            }
        }
        RandPart { rand_portion: res }
    }
}
