pub use autorand_derive::Random;
pub use rand;

use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet, VecDeque, LinkedList};
use std::hash::{Hash, BuildHasher};

use rand::{Rng, distributions::Alphanumeric};

const LEN_LIMIT: usize = 16;

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
        let length = rand::random::<usize>() % LEN_LIMIT;
        (0..)
            .map(|_| rand::thread_rng().sample(Alphanumeric))
            .take(length)
            .collect()
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

fn rand_length_iter<T: Random>() -> impl Iterator<Item=T> {
    let length = rand::random::<usize>() % LEN_LIMIT;
    rand_iter().take(length)
}

fn rand_iter<T: Random>() -> impl Iterator<Item=T> {
    (0..).map(|_| T::random())
}

macro_rules! impl_primitives {
    ($($t:ty,)*) => {
        $(
        impl Random for $t {
            fn random() -> Self {
                rand::random()
            }
        }
        )*
    };
}

impl_primitives!(
    u8, u16, u32, u64,
    i8, i16, i32, i64,
    f32, f64,
    usize, isize,
    char,
    bool,
);

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

impl_arrays!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,);

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