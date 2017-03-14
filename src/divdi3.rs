use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Divdi3 {
    a: i64,
    b: i64,
    c: i64,
}

impl TestCase for Divdi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_i64(rng);
        let b = util::gen_i64(rng);
        if b == 0 {
            return None;
        }
        let c = a / b;

        Some(Divdi3 { a, b, c })
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
use compiler_builtins::int::sdiv::__divdi3;

static TEST_CASES: &[((i64, i64), i64)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn divdi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __divdi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
