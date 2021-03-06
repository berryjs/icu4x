// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;
mod helpers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid::subtags::{Language, Region, Script, Variant};
use icu_locid::ParserError;

macro_rules! subtag_bench {
    ($c:expr, $name:expr, $subtag:ident, $data:expr) => {
        $c.bench_function(&format!("subtags/{}/parse", $name), |b| {
            b.iter(|| {
                for s in &$data.valid {
                    let _: $subtag = black_box(s).parse().unwrap();
                }
                for s in &$data.invalid {
                    let _: ParserError = black_box(s).parse::<$subtag>().unwrap_err();
                }
            })
        });
    };
}

fn subtags_bench(c: &mut Criterion) {
    let path = "./benches/fixtures/subtags.json";
    let data: fixtures::Subtags = helpers::read_fixture(path).expect("Failed to read a fixture");

    subtag_bench!(c, "language", Language, data.language);
    subtag_bench!(c, "script", Script, data.script);
    subtag_bench!(c, "region", Region, data.region);
    subtag_bench!(c, "variant", Variant, data.variant);
}

criterion_group!(benches, subtags_bench,);
criterion_main!(benches);
