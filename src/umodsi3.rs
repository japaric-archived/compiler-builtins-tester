use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Umodsi3 {
    a: u32,
    b: u32,
    c: u32,
}

impl TestCase for Umodsi3 {
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
        let c = a % b;

        Some(Umodsi3 { a, b, c })
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
use compiler_builtins::int::udiv::__umodsi3;

static TEST_CASES: &[((u32, u32), u32)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn umodsi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __umodsi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
