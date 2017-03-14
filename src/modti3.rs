use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Modti3 {
    a: i128,
    b: i128,
    c: i128,
}

impl TestCase for Modti3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_i128(rng);
        let b = util::gen_i128(rng);
        if b == 0 {
            return None;
        }
        let c = a % b;

        Some(Modti3 { a, b, c })
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
use compiler_builtins::int::sdiv::__modti3;

static TEST_CASES: &[((i128, i128), i128)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn modti3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __modti3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
