use autorand::Random;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::{Debug, Display};

#[derive(Random)]
struct U<T1, T2: Debug, T3> where T3: Display {
    a: T1,
    b: T2,
    c: T3,
    value: [T1; 16],
}

#[derive(Random)]
struct C<T1, T2: Debug, T3> where T3: Eq + Hash {
    a: Vec<T1>,
    b: Vec<T2>,
    c: HashMap<Vec<T3>, Vec<HashSet<T3>>>,
    value: (T1, T2, T3)
}

#[derive(Random)]
enum E<T1, T2: Debug, T3> where T3: Display + Eq + Hash {
    A,
    B { u: U<T1, T2, T3>, c: C<T1, T2, T3>, primitive: char },
    C(C<T1, T2, T3>)
}

#[test]
fn generate() {
    <U<u8, i32, i64>>::random();
    <C<(i16, i8, u32), [u8; 32], String>>::random();
    <E<Vec<u8>, char, bool>>::random();
}