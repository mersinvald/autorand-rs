use autorand::Random;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

// Formatting skipped because lack of trailing comma in the where-clause is a test-case
#[rustfmt::skip]
#[derive(Random)]
struct U<T1, T2: Debug, T3>
where
    T3: Display
{
    a: T1,
    b: T2,
    c: T3,
    value: [T1; 16],
}

#[derive(Random)]
struct C<T1, T2: Debug, T3>
where
    T3: Eq + Hash,
{
    a: Vec<T1>,
    b: Vec<T2>,
    c: HashMap<Vec<T3>, Vec<HashSet<T3>>>,
    value: (T1, T2, T3),
}

#[derive(Random)]
enum E<T1, T2: Debug, T3>
where
    T3: Display + Eq + Hash,
{
    A,
    B {
        u: U<T1, T2, T3>,
        c: C<T1, T2, T3>,
        primitive: char,
    },
    C(C<T1, T2, T3>),
}

#[test]
fn generate() {
    <U<u8, i32, i64>>::random();
    <C<(i16, i8, u32), [u8; 32], String>>::random();
    <E<Vec<u8>, char, bool>>::random();
}

fn test_transcode<N>(rounds: usize)
where
    N: Random + Serialize + DeserializeOwned + PartialEq + Debug,
{
    for _ in 0..rounds {
        let n = N::random();
        let n_json = serde_json::to_string(&n).unwrap();
        let n_value = serde_json::to_value(&n).unwrap();
        let n_dec = serde_json::from_str(&n_json).unwrap();
        let n_dec_value = serde_json::from_value(n_value).unwrap();
        assert_eq!(n, n_dec);
        assert_eq!(n, n_dec_value);
    }
}

#[test]
fn transcode_float_serde_json() {
    test_transcode::<f32>(100000);
    test_transcode::<f64>(100000);
}

#[test]
fn transcode_vec_float_serde_json() {
    test_transcode::<Vec<f32>>(10000);
    test_transcode::<Vec<f64>>(10000);
}
