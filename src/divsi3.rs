use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Divsi3 {
    a: i32,
    b: i32,
    c: i32,
}

impl TestCase for Divsi3 {
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

        Some(Divsi3 { a, b, c })
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
use compiler_builtins::int::sdiv::__divsi3;

static TEST_CASES: &[((i32, i32), i32)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn divsi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __divsi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
