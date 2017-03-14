#![feature(i128_type)]

extern crate rand;

use rand::Rng;

mod adddf3;
mod addsf3;
mod ashldi3;
mod ashlti3;
mod ashrdi3;
mod ashrti3;
mod divdi3;
mod divmoddi4;
mod divmodsi4;
mod divsi3;
mod divti3;
mod lshrdi3;
mod lshrti3;
mod moddi3;
mod modsi3;
mod modti3;
mod muldi3;
mod mulodi4;
mod mulosi4;
mod muloti4;
mod multi3;
mod powidf2;
mod powisf2;
mod subdf3;
mod subsf3;
mod udivdi3;
mod udivmoddi4;
mod udivmodsi4;
mod udivmodti4;
mod udivsi3;
mod udivti3;
mod umoddi3;
mod umodsi3;
mod umodti3;
mod util;

// use powidf2::Powidf2;
// use powisf2::Powisf2;
use adddf3::Adddf3;
use addsf3::Addsf3;
use ashldi3::Ashldi3;
use ashlti3::Ashlti3;
use ashrdi3::Ashrdi3;
use ashrti3::Ashrti3;
use divdi3::Divdi3;
use divmoddi4::Divmoddi4;
use divmodsi4::Divmodsi4;
use divsi3::Divsi3;
use divti3::Divti3;
use lshrdi3::Lshrdi3;
use lshrti3::Lshrti3;
use moddi3::Moddi3;
use modsi3::Modsi3;
use modti3::Modti3;
use muldi3::Muldi3;
use mulodi4::Mulodi4;
use mulosi4::Mulosi4;
use muloti4::Muloti4;
use multi3::Multi3;
use subdf3::Subdf3;
use subsf3::Subsf3;
use udivdi3::Udivdi3;
use udivmoddi4::Udivmoddi4;
use udivmodsi4::Udivmodsi4;
use udivmodti4::Udivmodti4;
use udivsi3::Udivsi3;
use udivti3::Udivti3;
use umoddi3::Umoddi3;
use umodsi3::Umodsi3;
use umodti3::Umodti3;

fn main() {
    const NTESTS: usize = 10_000;

    let rng = &mut rand::thread_rng();

    // float/add
    util::mk_file("adddf3", util::mk_tests::<Adddf3, _>(NTESTS, rng));
    util::mk_file("addsf3", util::mk_tests::<Addsf3, _>(NTESTS, rng));
    // FIXME powi doesn't work on thumb without aeabi_*mul
    // util::mk_file("powidf2", util::mk_tests::<Powidf2, _>(NTESTS, rng));
    // util::mk_file("powisf2", util::mk_tests::<Powisf2, _>(NTESTS, rng));
    util::mk_file("subdf3", util::mk_tests::<Subdf3, _>(NTESTS, rng));
    util::mk_file("subsf3", util::mk_tests::<Subsf3, _>(NTESTS, rng));

    // int/mul
    util::mk_file("muldi3", util::mk_tests::<Muldi3, _>(NTESTS, rng));
    util::mk_file("mulodi4", util::mk_tests::<Mulodi4, _>(NTESTS, rng));
    util::mk_file("mulosi4", util::mk_tests::<Mulosi4, _>(NTESTS, rng));
    util::mk_file("muloti4", util::mk_tests::<Muloti4, _>(NTESTS, rng));
    util::mk_file("multi3", util::mk_tests::<Multi3, _>(NTESTS, rng));

    // int/sdiv
    util::mk_file("divdi3", util::mk_tests::<Divdi3, _>(NTESTS, rng));
    util::mk_file("divmoddi4", util::mk_tests::<Divmoddi4, _>(NTESTS, rng));
    util::mk_file("divmodsi4", util::mk_tests::<Divmodsi4, _>(NTESTS, rng));
    util::mk_file("divsi3", util::mk_tests::<Divsi3, _>(NTESTS, rng));
    util::mk_file("divti3", util::mk_tests::<Divti3, _>(NTESTS, rng));
    util::mk_file("moddi3", util::mk_tests::<Moddi3, _>(NTESTS, rng));
    util::mk_file("modsi3", util::mk_tests::<Modsi3, _>(NTESTS, rng));
    util::mk_file("modti3", util::mk_tests::<Modti3, _>(NTESTS, rng));

    // int/shift
    util::mk_file("ashldi3", util::mk_tests::<Ashldi3, _>(NTESTS, rng));
    util::mk_file("ashlti3", util::mk_tests::<Ashlti3, _>(NTESTS, rng));
    util::mk_file("ashrdi3", util::mk_tests::<Ashrdi3, _>(NTESTS, rng));
    util::mk_file("ashrti3", util::mk_tests::<Ashrti3, _>(NTESTS, rng));
    util::mk_file("lshrdi3", util::mk_tests::<Lshrdi3, _>(NTESTS, rng));
    util::mk_file("lshrti3", util::mk_tests::<Lshrti3, _>(NTESTS, rng));

    // int/udiv
    util::mk_file("udivdi3", util::mk_tests::<Udivdi3, _>(NTESTS, rng));
    util::mk_file("udivmoddi4", util::mk_tests::<Udivmoddi4, _>(NTESTS, rng));
    util::mk_file("udivmodsi4", util::mk_tests::<Udivmodsi4, _>(NTESTS, rng));
    util::mk_file("udivmodti4", util::mk_tests::<Udivmodti4, _>(NTESTS, rng));
    util::mk_file("udivsi3", util::mk_tests::<Udivsi3, _>(NTESTS, rng));
    util::mk_file("udivti3", util::mk_tests::<Udivti3, _>(NTESTS, rng));
    util::mk_file("umoddi3", util::mk_tests::<Umoddi3, _>(NTESTS, rng));
    util::mk_file("umodsi3", util::mk_tests::<Umodsi3, _>(NTESTS, rng));
    util::mk_file("umodti3", util::mk_tests::<Umodti3, _>(NTESTS, rng));
}

pub trait TestCase {
    fn generate<R>(rng: &mut R) -> Option<Self>
    where
        R: Rng,
        Self: Sized;
    fn stringify(&self, buffer: &mut String);
    fn prologue() -> &'static str;
    fn epilogue() -> &'static str;
}
