use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Mulodi4 {
    a: i64,
    b: i64,
    c: i64,
    overflow: u32,
}

impl TestCase for Mulodi4 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
    {
        let a = util::gen_i64(rng);
        let b = util::gen_i64(rng);
        let c = a.wrapping_mul(b);
        let overflow = if a.checked_mul(b).is_some() { 0 } else { 1 };

        Some(Mulodi4 {
                 a,
                 b,
                 c,
                 overflow,
             })
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
        assert_eq!(((a, b), (c, overflow)), ((a, b), (c_, overflow_)));
    }
}
"
    }
}
