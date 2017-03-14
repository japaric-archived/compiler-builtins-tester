use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Ashrdi3 {
    a: i64,
    b: u32,
    c: i64,
}

impl TestCase for Ashrdi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_i64(rng);
        let b = (rng.gen::<u8>() % 64) as u32;
        let c = a >> b;

        Some(Ashrdi3 { a, b, c })
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
use compiler_builtins::int::shift::__ashrdi3;

static TEST_CASES: &[((i64, u32), i64)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn ashrdi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __ashrdi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
