extern crate rand;

use std::collections::HashSet;
use std::fmt::Write;
use std::fs::File;
use std::hash::Hash;

use rand::Rng;

#[derive(Eq, Hash, PartialEq)]
struct Mulodi4 {
    a: i64,
    b: i64,
    c: i64,
    overflow: u32,
}

impl TestCase for Mulodi4 {
    fn generate<R>(rng: &mut R) -> Option<Self>
        where R: Rng
    {
        let a: i64 = if rng.gen() {
            rng.gen::<i32>() as i64
        } else {
            rng.gen()
        };
        let b: i64 = if rng.gen() {
            rng.gen::<i32>() as i64
        } else {
            rng.gen()
        };
        let c = a.wrapping_mul(b);
        let overflow = if a.checked_mul(b).is_some() { 0 } else { 1 };

        Some(Mulodi4 {
                 a,
                 b,
                 c,
                 overflow,
             })
    }

    fn prologue() -> &'static str {
        "
use compiler_builtins::int::mul::__mulodi4;

static TEST_CASES: &[((i64, i64), (i64, i32))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn mulodi4() {
    let mut overflow_ = 2;
    for &((a, b), (c, overflow)) in TEST_CASES {
        let c_ = __mulodi4(a, b, &mut overflow_);
        assert_eq!((c_, overflow_), (c, overflow));
    }
}
"
    }

    fn stringify(&self, buffer: &mut String) {
        writeln!(buffer,
                 "(({a}, {b}), ({c}, {overflow})),",
                 a = self.a,
                 b = self.b,
                 c = self.c,
                 overflow = self.overflow)
                .unwrap();
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Muldi3 {
    a: u64,
    b: u64,
    c: u64,
}

impl TestCase for Muldi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
        where R: Rng,
              Self: Sized
    {
        let a: u64 = rng.gen();
        let b: u64 = rng.gen();
        let c = a.wrapping_mul(b);

        Some(Muldi3 { a, b, c })
    }

    fn stringify(&self, buffer: &mut String) {
        writeln!(buffer,
                 "(({a}, {b}), {c}),",
                 a = self.a,
                 b = self.b,
                 c = self.c)
                .unwrap();
    }

    fn prologue() -> &'static str {
        "
use compiler_builtins::int::mul::__muldi3;

static TEST_CASES: &[((u64, u64), u64)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn muldi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __muldi3(a, b);
        assert_eq!(c, c_);
    }
}
"
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    file("mulodi4", mk_tests::<Mulodi4, _>(10_000, &mut rng));
    file("muldi3", mk_tests::<Muldi3, _>(10_000, &mut rng));
}

fn file(name: &str, contents: String) {
    use std::io::Write;

    File::create(format!("tester/tests/{}.rs", name))
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}

trait TestCase {
    fn generate<R>(rng: &mut R) -> Option<Self>
        where R: Rng,
              Self: Sized;
    fn stringify(&self, buffer: &mut String);
    fn prologue() -> &'static str;
    fn epilogue() -> &'static str;
}

const PROLOGUE: &'static str = r#"
#![feature(compiler_builtins_lib)]
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

fn mk_tests<T, R>(mut n: usize, rng: &mut R) -> String
    where T: Eq + Hash + TestCase,
          R: Rng
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
