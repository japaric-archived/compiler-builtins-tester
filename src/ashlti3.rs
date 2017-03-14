use std::fmt::Write;

use rand::Rng;

use util;
use TestCase;

#[derive(Eq, Hash, PartialEq)]
pub struct Ashlti3 {
    a: u128,
    b: u32,
    c: u128,
}

impl TestCase for Ashlti3 {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized,
    {
        let a = util::gen_u128(rng);
        let b = (rng.gen::<u8>() % 128) as u32;
        let c = a << b;

        Some(Ashlti3 { a, b, c })
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
use compiler_builtins::int::shift::__ashlti3;

static TEST_CASES: &[((u128, u32), u128)] = &[
"
    }

    fn epilogue() -> &'static str {
        "
];

#[test]
fn ashlti3() {
    for &((a, b), c) in TEST_CASES {
        let c_ = __ashlti3(a, b);
        assert_eq!(((a, b), c), ((a, b), c_));
    }
}
"
    }
}
