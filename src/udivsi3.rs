use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivsi3 {
    a: u32,
    b: u32,
    c: u32,
}

impl TestCase for Udivsi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u32(rng);
        let b = util::gen_u32(rng);
        if b == 0 {
            return None;
        }
        let c = a / b;

        Some(Udivsi3 { a, b, c })
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
use compiler_builtins::int::udiv::__udivsi3;

static TEST_CASES: &[((u32, u32), u32)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivsi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __udivsi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
