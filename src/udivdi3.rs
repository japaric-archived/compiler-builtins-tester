use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivdi3 {
    a: u64,
    b: u64,
    c: u64,
}

impl TestCase for Udivdi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u64(rng);
        let b = util::gen_u64(rng);
        if b == 0 {
            return None;
        }
        let c = a / b;

        Some(Udivdi3 { a, b, c })
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
use compiler_builtins::int::udiv::__udivdi3;

static TEST_CASES: &[((u64, u64), u64)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivdi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __udivdi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
