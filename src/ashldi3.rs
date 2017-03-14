use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Ashldi3 {
    a: u64,
    b: u32,
    c: u64,
}

impl TestCase for Ashldi3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u64(rng);
        let b = (rng.gen::<u8>() % 64) as u32;
        let c = a << b;

        Some(Ashldi3 { a, b, c })
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
use compiler_builtins::int::shift::__ashldi3;

static TEST_CASES: &[((u64, u32), u64)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn ashldi3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __ashldi3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
