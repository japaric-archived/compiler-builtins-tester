use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Muloti4 {
    a: i128,
    b: i128,
    c: i128,
    overflow: u32,
}

impl TestCase for Muloti4 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
    {
        let a = util::gen_i128(rng);
        let b = util::gen_i128(rng);
        let c = a.wrapping_mul(b);
        let overflow = if a.checked_mul(b).is_some() { 0 } else { 1 };

        Some(Muloti4 {
                 a,
                 b,
                 c,
                 overflow,
             })
    }

    fn prologue() -> &'static str {
        "
use compiler_builtins::int::mul::__muloti4;

static TEST_CASES: &[((i128, i128), (i128, i32))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn muloti4() {
    let mut overflow_ = 2;
    for &((a, b), (c, overflow)) in TEST_CASES {
        let c_ = __muloti4(a, b, &mut overflow_);
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
