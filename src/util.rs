#![allow(dead_code)]

use std::{f32, f64, i16, i32, i64};
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::mem;

use rand::Rng;

use TestCase;

macro_rules! gen_float {
    ($name:ident,
     $fty:ident,
     $uty:ident,
     $bits:expr,
     $significand_bits:expr) => {
        pub fn $name<R>(rng: &mut R) -> $fty
        where
            R: Rng,
        {
            const BITS: u8 = $bits;
            const SIGNIFICAND_BITS: u8 = $significand_bits;
            const EXPONENT_BITS: u8 = BITS - SIGNIFICAND_BITS - 1;

            const SIGNIFICAND_MASK: $uty = (1 << SIGNIFICAND_BITS) - 1;
            const SIGN_MASK: $uty = (1 << (BITS - 1));
            const EXPONENT_MASK: $uty = !(SIGN_MASK | SIGNIFICAND_MASK);

            fn mk_f32(sign: bool, exponent: $uty, significand: $uty) -> $fty {
                unsafe {
                    mem::transmute(((sign as $uty) << (BITS - 1)) |
                                ((exponent & EXPONENT_MASK) <<
                                    SIGNIFICAND_BITS) |
                                (significand & SIGNIFICAND_MASK))
                }
            }


            if rng.gen_weighted_bool(10) {
                // Special values
                *rng.choose(&[-0.0,
                              0.0,
                              $fty::NAN,
                              $fty::INFINITY,
                              -$fty::INFINITY])
                    .unwrap()
            } else if rng.gen_weighted_bool(10) {
                // NaN patterns
                mk_f32(rng.gen(), rng.gen(), 0)
            } else if rng.gen() {
                // Denormalized
                mk_f32(rng.gen(), 0, rng.gen())
            } else {
                // Random anything
                mk_f32(rng.gen(), rng.gen(), rng.gen())
            }
        }
    }
}

gen_float!(gen_f32, f32, u32, 32, 23);
gen_float!(gen_f64, f64, u64, 64, 52);

macro_rules! gen_int {
    ($name:ident, $ity:ident, $hty:ident) => {
        pub fn $name<R>(rng: &mut R) -> $ity
            where
            R: Rng,
        {
            let mut mk = || if rng.gen_weighted_bool(10) {
                *rng.choose(&[$hty::MAX, 0, $hty::MIN]).unwrap()
            } else {
                rng.gen::<$hty>()
            };
            unsafe { mem::transmute([mk(), mk()]) }
        }

    }
}

gen_int!(gen_i32, i32, i16);
gen_int!(gen_i64, i64, i32);
gen_int!(gen_i128, i128, i64);

pub fn gen_u32<R>(rng: &mut R) -> u32
where
    R: Rng,
{
    gen_i32(rng) as u32
}

pub fn gen_u64<R>(rng: &mut R) -> u64
where
    R: Rng,
{
    gen_i64(rng) as u64
}

pub fn gen_u128<R>(rng: &mut R) -> u128
where
    R: Rng,
{
    gen_i128(rng) as u128
}

pub fn to_f32(x: u32) -> f32 {
    unsafe { mem::transmute(x) }
}

pub fn to_f64(x: u64) -> f64 {
    unsafe { mem::transmute(x) }
}

pub fn mk_file(name: &str, contents: String) {
    use std::io::Write;

    File::create(format!("tester/tests/{}.rs", name))
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}

const PROLOGUE: &'static str = r#"
#![feature(compiler_builtins_lib)]
#![feature(i128_type)]
#![cfg_attr(all(target_arch = "arm",
                not(any(target_env = "gnu", target_env = "musl")),
                target_os = "linux",
                test), no_std)]

extern crate compiler_builtins;

// test runner
#[cfg(all(target_arch = "arm",
          not(any(target_env = "gnu", target_env = "musl")),
          target_os = "linux",
          test))]
extern crate utest_cortex_m_qemu;

// overrides `panic!`
#[cfg(all(target_arch = "arm",
          not(any(target_env = "gnu", target_env = "musl")),
          target_os = "linux",
          test))]
#[macro_use]
extern crate utest_macros;
"#;

pub fn mk_tests<T, R>(mut n: usize, rng: &mut R) -> String
where
    T: Eq + Hash + TestCase,
    R: Rng,
{
    let mut buffer = PROLOGUE.to_owned();
    buffer.push_str(T::prologue());
    let mut cases = HashSet::new();
    while n != 0 {
        if let Some(case) = T::generate(rng) {
            if cases.contains(&case) {
                continue;
            }
            case.stringify(&mut buffer);
            n -= 1;
            cases.insert(case);
        }
    }
    buffer.push_str(T::epilogue());
    buffer
}

pub fn to_u32(x: f32) -> u32 {
    unsafe { mem::transmute(x) }
}

pub fn to_u64(x: f64) -> u64 {
    unsafe { mem::transmute(x) }
}
