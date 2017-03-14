use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Powidf2 {
    a: u64,
    b: i32,
    c: u64,
}

impl TestCase for Powidf2 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_f64(rng);
        let b = util::gen_i32(rng);
        let c = a.powi(b);
        // TODO accept NaNs. We don't do that right now because we can't check
        // for NaN-ness on the thumb targets
        if a.is_nan() || c.is_nan() {
            return None;
        }

        Some(Powidf2 {
                 a: util::to_u64(c),
                 b,
                 c: util::to_u64(c),
             })
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
        r#"
#[cfg(all(target_arch = "arm",
          not(any(target_env = "gnu", target_env = "musl")),
          target_os = "linux",
          test))]
use core::mem;
#[cfg(not(all(target_arch = "arm",
              not(any(target_env = "gnu", target_env = "musl")),
              target_os = "linux",
              test)))]
use std::mem;
use compiler_builtins::float::pow::__powidf2;

fn mk_f64(x: u64) -> f64 {
    unsafe { mem::transmute(x) }
}

fn to_u64(x: f64) -> u64 {
    unsafe { mem::transmute(x) }
}

static TEST_CASES: &[((u64, i32), u64)] = &[
"#
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn powidf2() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __powidf2(mk_f64(a), b);
        assert_eq!(((a, b), c), ((a, b), to_u64(c_)));
    }
}
"
    }
}
