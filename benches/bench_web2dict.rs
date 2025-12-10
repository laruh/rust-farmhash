use criterion::{Criterion, criterion_group, criterion_main};
use farmhash::{self, FarmHasher};
use fnv::FnvHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::hint::black_box;
use std::io::prelude::*;
use std::path::Path;

// Macros to insert into other macros to test with Hash trait or with a direct function
macro_rules! hash_hashing {
    ($data:ident, $hasher:expr) => {{
        let mut hasher = $hasher;
        $data.hash(&mut hasher);
        black_box(hasher.finish());
    }};
}

macro_rules! direct_hashing_str {
    ($data:ident, $hasher:expr) => {{
        black_box($hasher($data.as_bytes()));
    }};
}

macro_rules! direct_hashing_u8 {
    ($data:ident, $hasher:expr) => {{
        black_box($hasher($data));
    }};
}

// Dictionary benchmark
fn bench_dicts(c: &mut Criterion) {
    let path = Path::new("benches/sample-dict");
    let display = path.display();

    // Open file in read-only mode
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open '{}': {}", display, e),
        Ok(file) => file,
    };

    // Read all contents to string
    let mut dict = String::new();
    if let Err(e) = file.read_to_string(&mut dict) {
        panic!("Couldn't read '{}': {}", display, e);
    }

    let mut group = c.benchmark_group("dict");

    #[allow(deprecated)]
    group.bench_function("dict_sip24", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                hash_hashing!(s, std::hash::SipHasher::default());
            }
        });
    });

    group.bench_function("dict_default_hasher", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                hash_hashing!(s, std::hash::DefaultHasher::default());
            }
        });
    });

    group.bench_function("dict_fnv", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                hash_hashing!(s, FnvHasher::default());
            }
        });
    });

    group.bench_function("dict_farm", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                hash_hashing!(s, FarmHasher::default());
            }
        });
    });

    group.bench_function("dict_farm_direct", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                direct_hashing_str!(s, farmhash::hash64);
            }
        });
    });

    group.bench_function("dict_farm_fingerprint64", |b| {
        b.iter(|| {
            for s in dict.split('\n') {
                direct_hashing_str!(s, farmhash::fingerprint64);
            }
        });
    });

    group.finish();
}

// Lorem Ipsum benchmark
fn bench_lorem(c: &mut Criterion) {
    let data = [
        "Lorem",
        "ipsum",
        "dolor",
        "sit",
        "amet,",
        "consetetur",
        "sadipscing",
        "elitr,",
        "sed",
        "diam",
        "nonumy",
        "eirmod",
        "tempor",
        "invidunt",
        "ut",
        "labore",
        "et",
        "dolore",
        "magna",
        "aliquyam",
        "erat,",
        "sed",
        "diam",
        "voluptua.",
    ];

    let mut group = c.benchmark_group("lorem");

    #[allow(deprecated)]
    group.bench_function("lorem_sip24", |b| {
        b.iter(|| {
            for s in &data {
                hash_hashing!(s, std::hash::SipHasher::default());
            }
        });
    });

    group.bench_function("lorem_default_hasher", |b| {
        b.iter(|| {
            for s in &data {
                hash_hashing!(s, std::hash::DefaultHasher::default());
            }
        });
    });

    group.bench_function("lorem_fnv", |b| {
        b.iter(|| {
            for s in &data {
                hash_hashing!(s, FnvHasher::default());
            }
        });
    });

    group.bench_function("lorem_farm", |b| {
        b.iter(|| {
            for s in &data {
                hash_hashing!(s, FarmHasher::default());
            }
        });
    });

    group.bench_function("lorem_farm_direct", |b| {
        b.iter(|| {
            for s in &data {
                direct_hashing_str!(s, farmhash::hash64);
            }
        });
    });

    group.bench_function("lorem_farm_fingerprint64", |b| {
        b.iter(|| {
            for s in &data {
                direct_hashing_str!(s, farmhash::fingerprint64);
            }
        });
    });

    group.finish();
}

// Pseudo random data benchmark
fn bench_pseudorand(c: &mut Criterion) {
    let path = Path::new("benches/pseudo-random-data.bin");
    let display = path.display();

    // Open file in read-only mode
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open '{}': {}", display, e),
        Ok(file) => file,
    };

    // Read all contents to vec
    let mut data = Vec::new();
    if let Err(e) = file.read_to_end(&mut data) {
        panic!("Couldn't read '{}': {}", display, e);
    }

    let mut group = c.benchmark_group("pseudo-random");

    #[allow(deprecated)]
    group.bench_function("pseudorand_big_sip24", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                hash_hashing!(chunk, std::hash::SipHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_big_default_hasher", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                hash_hashing!(chunk, std::hash::DefaultHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_big_fnv", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                hash_hashing!(chunk, FnvHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_big_farm", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                hash_hashing!(chunk, FarmHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_big_farm_direct", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                direct_hashing_u8!(chunk, farmhash::hash64);
            }
        });
    });

    group.bench_function("pseudorand_big_farm_fingerprint64", |b| {
        b.iter(|| {
            for chunk in data.chunks(512) {
                direct_hashing_u8!(chunk, farmhash::fingerprint64);
            }
        });
    });

    #[allow(deprecated)]
    group.bench_function("pseudorand_small_sip24", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                hash_hashing!(chunk, std::hash::SipHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_small_default_hasher", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                hash_hashing!(chunk, std::hash::DefaultHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_small_fnv", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                hash_hashing!(chunk, FnvHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_small_farm", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                hash_hashing!(chunk, FarmHasher::default());
            }
        });
    });

    group.bench_function("pseudorand_small_farm_direct", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                direct_hashing_u8!(chunk, farmhash::hash64);
            }
        });
    });

    group.bench_function("pseudorand_small_farm_fingerprint64", |b| {
        b.iter(|| {
            for chunk in data.chunks(4) {
                direct_hashing_u8!(chunk, farmhash::fingerprint64);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_dicts, bench_lorem, bench_pseudorand);
criterion_main!(benches);
