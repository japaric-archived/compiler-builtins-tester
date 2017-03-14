use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Multi3 {
    a: i128,
    b: i128,
    c: i128,
}

impl TestCase for Multi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_i128(rng);
        let b = util::gen_i128(rng);
        let c = a.wrapping_mul(b);

        Some(Multi3 { a, b, c })
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
use compiler_builtins::int::mul::__multi3;

static TEST_CASES: &[((i128, i128), i128)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn multi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __multi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
