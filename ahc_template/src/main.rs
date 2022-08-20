#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

// ---------- set const values ----------
// build.rs set const values with "--features env_const"
#[cfg(feature = "env_const")]
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

#[cfg(not(feature = "env_const"))]
pub mod params {
    pub const SAMPLE_PARAM: i64 = 1234;
}
#[cfg(not(feature = "env_const"))]
use params::*;

use crate::mytool::TimeManager;
// --------------------------------------

#[cfg(not(feature = "mylocal"))]
const TL: u64 = 2850;

#[cfg(feature = "mylocal")]
const TL: u64 = 1500;

// #[fastout]
fn main() {
    let mut tm = TimeManager::new();
    let mut myrng = mytool::MyRng::new();
    input! {
        //
    }
    // output "Score = {}" by eprintln! for test.py
    let score = -(SAMPLE_PARAM - 100).pow(2);
    eprintln!("Score = {}", score);
}

pub mod mytool {
    use rand::prelude::*;
    use std::fmt;
    use std::time::Instant;
    pub struct TimeManager {
        count: u64,
        start: Instant,
        last_time: u64,
    }
    impl fmt::Display for TimeManager {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.count)
        }
    }
    impl TimeManager {
        const COUNT_PER_MEASURE: u64 = 1000;
        pub fn new() -> Self {
            TimeManager {
                count: 0,
                start: Instant::now(),
                last_time: 0,
            }
        }
        pub fn check_time(&mut self, time: u64, force: bool) -> bool {
            self.count += 1;
            if force || self.count > Self::COUNT_PER_MEASURE {
                self.count = 0;
                self.last_time = self.start.elapsed().as_millis() as u64;
            }
            self.last_time < time
        }
        pub fn get_time(&self) -> u64 {
            self.start.elapsed().as_millis() as u64
        }
    }
    pub struct MyRng {
        rng: SmallRng,
    }
    impl MyRng {
        pub fn new() -> Self {
            MyRng {
                rng: SmallRng::seed_from_u64(4445),
            }
        }
        pub fn get_int(&mut self, left: i32, right: i32) -> i32 {
            //! get [left, right] ( not [left, right)  )
            self.rng.gen_range(left, right + 1)
        }
        pub fn get_percent(&mut self) -> f64 {
            self.rng.gen::<f64>()
        }
        pub fn shuffle<T>(&mut self, v: &mut [T]) {
            v.shuffle(&mut self.rng);
        }
        pub fn choose<T: Copy>(&mut self, v: &[T]) -> T {
            *v.choose(&mut self.rng).unwrap()
        }
    }
}
