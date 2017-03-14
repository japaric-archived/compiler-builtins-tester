use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Divmoddi4 {
    a: i64,
    b: i64,
    c: i64,
    rem: i64,
}

impl TestCase for Divmoddi4 {
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
        let rem = a % b;

        Some(Divmoddi4 { a, b, c, rem })
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
use compiler_builtins::int::sdiv::__divmoddi4;

static TEST_CASES: &[((i64, i64), (i64, i64))] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn divmoddi4() {
    for &((a, b), (c, rem)) in TEST_CASES {
        let mut rem_ = 0;
        let c_ = __divmoddi4(a, b, &mut rem_);
        assert_eq!(((a, b), (c, rem)), ((a, b), (c_, rem_)));
    }
}
"
    }
}
