use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Muldi3 {
    a: u64,
    b: u64,
    c: u64,
}

impl TestCase for Muldi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u64(rng);
        let b = util::gen_u64(rng);
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
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
