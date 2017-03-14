use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivmodsi4 {
    a: u32,
    b: u32,
    c: u32,
    rem: u32,
}

impl TestCase for Udivmodsi4 {
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
        let rem = a % b;

        Some(Udivmodsi4 { a, b, c, rem })
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
use compiler_builtins::int::udiv::__udivmodsi4;

static TEST_CASES: &[((u32, u32), (u32, u32))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivmodsi4() {
    for &((a, b), (c, rem)) in TEST_CASES {
        let mut rem_ = 0;
        let c_ = __udivmodsi4(a, b, Some(&mut rem_));
        assert_eq!(((a, b), (c, rem)), ((a, b), (c_, rem_)));
    }
}
"
    }
}
