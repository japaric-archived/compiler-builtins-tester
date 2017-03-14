use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Mulosi4 {
    a: i32,
    b: i32,
    c: i32,
    overflow: u32,
}

impl TestCase for Mulosi4 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
    {
        let a = util::gen_i32(rng);
        let b = util::gen_i32(rng);
        let c = a.wrapping_mul(b);
        let overflow = if a.checked_mul(b).is_some() { 0 } else { 1 };

        Some(Mulosi4 {
                 a,
                 b,
                 c,
                 overflow,
             })
    }

    fn prologue() -> &'static str {
        "
use compiler_builtins::int::mul::__mulosi4;

static TEST_CASES: &[((i32, i32), (i32, i32))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn mulosi4() {
    let mut overflow_ = 2;
    for &((a, b), (c, overflow)) in TEST_CASES {
        let c_ = __mulosi4(a, b, &mut overflow_);
        assert_eq!(((a, b), (c, overflow)), ((a, b), (c_, overflow_)));
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
