use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivmodti4 {
    a: u128,
    b: u128,
    c: u128,
    rem: u128,
}

impl TestCase for Udivmodti4 {
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
        let rem = a % b;

        Some(Udivmodti4 { a, b, c, rem })
    }

    fn stringify(&self, buffer: &mut String) {
        writeln!(buffer,
                 "(({a}, {b}), ({c}, {rem})),",
                 a = self.a,
                 b = self.b,
                 c = self.c,
                 rem = self.rem)
                .unwrap();
    }

    fn prologue() -> &'static str {
        "
use compiler_builtins::int::udiv::__udivmodti4;

static TEST_CASES: &[((u128, u128), (u128, u128))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivmodti4() {
    for &((a, b), (c, rem)) in TEST_CASES {
        let mut rem_ = 0;
        let c_ = __udivmodti4(a, b, Some(&mut rem_));
        assert_eq!(((a, b), (c, rem)), ((a, b), (c_, rem_)));
    }
}
"
    }
}
