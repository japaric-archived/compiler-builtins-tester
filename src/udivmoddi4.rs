use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Udivmoddi4 {
    a: u64,
    b: u64,
    c: u64,
    rem: u64,
}

impl TestCase for Udivmoddi4 {
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
        let rem = a % b;

        Some(Udivmoddi4 { a, b, c, rem })
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
use compiler_builtins::int::udiv::__udivmoddi4;

static TEST_CASES: &[((u64, u64), (u64, u64))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn udivmoddi4() {
    for &((a, b), (c, rem)) in TEST_CASES {
        let mut rem_ = 0;
        let c_ = __udivmoddi4(a, b, Some(&mut rem_));
        assert_eq!(((a, b), (c, rem)), ((a, b), (c_, rem_)));
    }
}
"
    }
}
