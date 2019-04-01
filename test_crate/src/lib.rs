use autorand::Random;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::{Debug, Display};

#[derive(Random)]
struct U<A, B: Debug, C> where C: Display {
    a: A,
    b: B,
    c: C,
    value: [A; 16],
}

#[derive(Random)]
struct C<A, B: Debug, D> where D: Eq + Hash {
    a: Vec<A>,
    b: Vec<B>,
    d: HashMap<Vec<D>, Vec<HashSet<D>>>,
    value: (A, B, D)
}

#[derive(Random)]
enum E<A, B: Debug, D> where D: Display {
    A,
    B { u: U<A, B, D>, c: C<A, B, D>, primitive: char },
    C(C<A, B, D>)
}