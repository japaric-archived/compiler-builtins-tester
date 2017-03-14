use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivti3 {
    a: u128,
    b: u128,
    c: u128,
}

impl TestCase for Udivti3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u128(rng);
        let b = util::gen_u128(rng);
        if b == 0 {
            return None;
        }
        let c = a / b;

        Some(Udivti3 { a, b, c })
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
use compiler_builtins::int::udiv::__udivti3;

static TEST_CASES: &[((u128, u128), u128)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivti3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __udivti3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
