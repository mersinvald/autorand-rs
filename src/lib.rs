pub use autorand_derive::Random;
pub use rand;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};

use rand::{distributions::Alphanumeric, Rng};

const LEN_LIMIT: usize = 16;

const UINT_LIMIT: usize = u16::max_value() as usize;
const INT_LOWER_LIMIT: isize = 0 - (UINT_LIMIT as isize);
const INT_UPPER_LIMIT: isize = UINT_LIMIT as isize;

pub trait Random: Sized {
    fn random() -> Self;
}

impl<T: Random> Random for Option<T> {
    fn random() -> Self {
        if rand::random::<bool>() {
            Some(T::random())
        } else {
            None
        }
    }
}

impl Random for String {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(0, LEN_LIMIT);
        rng.sample_iter(&Alphanumeric).take(length).collect()
    }
}

impl<T: Random> Random for Vec<T> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

impl<K: Random + Eq + Hash, V: Random, S: BuildHasher + Default> Random for HashMap<K, V, S> {
    fn random() -> Self {
        rand_length_iter::<(K, V)>().collect()
    }
}

impl<K: Random + Ord, V: Random> Random for BTreeMap<K, V> {
    fn random() -> Self {
        rand_length_iter::<(K, V)>().collect()
    }
}

impl<T: Random + Eq + Hash, S: BuildHasher + Default> Random for HashSet<T, S> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

impl<T: Random + Ord> Random for BTreeSet<T> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

impl<T: Random> Random for VecDeque<T> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

impl<T: Random> Random for LinkedList<T> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

fn rand_length_iter<T: Random>() -> impl Iterator<Item = T> {
    let length = rand::thread_rng().gen_range(0, LEN_LIMIT);
    rand_iter().take(length)
}

fn rand_iter<T: Random>() -> impl Iterator<Item = T> {
    (0..).map(|_| T::random())
}

impl Random for f32 {
    fn random() -> Self {
        let base = rand::random::<f32>();
        (base * 1000.0).ceil() / 1000.0
    }
}

impl Random for f64 {
    fn random() -> Self {
        let base = rand::random::<f64>();
        (base * 1000.0).ceil() / 1000.0
    }
}

macro_rules! impl_primitives_unsigned {
    ($($t:tt,)*) => {
        $(
        impl Random for $t {
            fn random() -> Self {
                let mut rng = rand::thread_rng();
                if cfg!(not(feature = "limited-integers")) || ($t::max_value() as usize) < UINT_LIMIT {
                    rng.gen_range(0, $t::max_value())
                } else {
                    rng.gen_range(0, UINT_LIMIT as $t)
                }
            }
        }
        )*
    };
}

macro_rules! impl_primitives_signed {
    ($($t:tt,)*) => {
        $(
        impl Random for $t {
            fn random() -> Self {
                let mut rng = rand::thread_rng();
                if cfg!(not(feature = "limited-integers")) || ($t::max_value() as isize) < INT_UPPER_LIMIT {
                    rng.gen_range($t::min_value(), $t::max_value())
                } else {
                    rng.gen_range(INT_LOWER_LIMIT as $t, INT_UPPER_LIMIT as $t)
                }
            }
        }
        )*
    };
}

#[rustfmt::skip]
impl_primitives_signed! {
    i8, i16, i32, i64, isize,
}

impl_primitives_unsigned! {
    u8, u16, u32, u64, usize,
}

impl Random for char {
    fn random() -> Self {
        rand::random()
    }
}

impl Random for bool {
    fn random() -> Self {
        rand::random()
    }
}

macro_rules! impl_arrays {
    ($($s:expr,)*) => {
        $(
        impl<T: Random> Random for [T; $s] {
            fn random() -> Self {
                unsafe {
                    let mut array: [T; $s] = std::mem::uninitialized();
                    for i in 0..$s {
                        std::ptr::write(&mut array[i], T::random());
                    }
                    array
                }
            }
        }
        )*
    };
}

#[rustfmt::skip]
impl_arrays!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32,
    64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
);

macro_rules! impl_tuples {
    ($([$($t:tt)*],)*) => {
        $(
        impl<$($t:Random,)*> Random for ($($t,)*) {
            fn random() -> Self {
                ($($t::random(),)*)
            }
        }
        )*
    };
}

impl_tuples!(
    [A],
    [A B],
    [A B C],
    [A B C D],
    [A B C D E],
    [A B C D E F],
    [A B C D E F G],
    [A B C D E F G H],
    [A B C D E F G H I],
    [A B C D E F G H I J],
    [A B C D E F G H I J K],
);

#[cfg(feature = "json")]
impl Random for serde_json::Map<String, serde_json::Value> {
    fn random() -> Self {
        rand_length_iter().collect()
    }
}

#[cfg(feature = "json")]
impl Random for serde_json::Number {
    fn random() -> Self {
        serde_json::Number::from_f64(Random::random()).unwrap()
    }
}

#[cfg(feature = "json")]
#[cfg(not(feature = "json-value-always-null"))]
impl Random for serde_json::Value {
    fn random() -> Self {
        use serde_json::Value;
        let variant = rand::thread_rng().gen_range(0u8, 6);
        match variant {
            0 => Value::Number(Random::random()),
            1 => Value::Bool(Random::random()),
            2 => Value::String(Random::random()),
            3 => Value::Array(Random::random()),
            4 => Value::Null,
            5 => Value::Object(Random::random()),
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "json")]
#[cfg(feature = "json-value-always-null")]
impl Random for serde_json::Value {
    fn random() -> Self {
        use serde_json::Value;
        Value::Null
    }
}
