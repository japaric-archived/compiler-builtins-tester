use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Divmodsi4 {
    a: i32,
    b: i32,
    c: i32,
    rem: i32,
}

impl TestCase for Divmodsi4 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_i32(rng);
        let b = util::gen_i32(rng);
        if b == 0 {
            return None;
        }
        let c = a / b;
        let rem = a % b;

        Some(Divmodsi4 { a, b, c, rem })
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
use compiler_builtins::int::sdiv::__divmodsi4;

static TEST_CASES: &[((i32, i32), (i32, i32))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn divmodsi4() {
    for &((a, b), (c, rem)) in TEST_CASES {
        let mut rem_ = 0;
        let c_ = __divmodsi4(a, b, &mut rem_);
        assert_eq!(((a, b), (c, rem)), ((a, b), (c_, rem_)));
    }
}
"
    }
}
