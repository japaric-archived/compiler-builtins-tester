use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Subsf3 {
    a: u32,
    b: u32,
    c: u32,
}

impl TestCase for Subsf3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_f32(rng);
        let b = util::gen_f32(rng);
        let c = a - b;
        // TODO accept NaNs. We don't do that right now because we can't check
        // for NaN-ness on the thumb targets (due to missing intrinsics)
        if a.is_nan() || b.is_nan() || c.is_nan() {
            return None;
        }

        Some(Subsf3 {
                 a: util::to_u32(a),
                 b: util::to_u32(b),
                 c: util::to_u32(c),
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
use compiler_builtins::float::sub::__subsf3;

fn mk_f32(x: u32) -> f32 {
    unsafe { mem::transmute(x) }
}

fn to_u32(x: f32) -> u32 {
    unsafe { mem::transmute(x) }
}

static TEST_CASES: &[((u32, u32), u32)] = &[
"#
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn subsf3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __subsf3(mk_f32(a), mk_f32(b));
        assert_eq!(((a, b), c), ((a, b), to_u32(c_)));
    }
}
"
    }
}
