use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::BitXor;

pub struct Pair<'a, T>
where T: PartialEq + Eq
{
    a: &'a T,
    b: &'a T
}

impl<'a, T> Pair<'a, T>
where T: PartialEq + Eq
{
    pub fn new(a: &'a T, b: &'a T) -> Self {
        Pair{
            a,
            b
        }
    }

    pub fn has(&self, value: &T) -> bool {
        value == self.a || value == self.b
    }

    pub fn other(&self, value: &T) -> &T {
        if value == self.a {
            self.b
        } else {
            self.a
        }
    }
}

impl<T> Debug for Pair<'_, T>
where T: PartialEq + Eq + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} <=> {:?}", self.a, self.b)
    }
}

impl<T> PartialEq for Pair<'_, T>
where T: PartialEq + Eq {
    fn eq(&self, other: &Self) -> bool {
        if self.a == other.a {
            self.b == self.b
        } else if self.b == other.a {
            self.a == other.b
        } else {
            false
        }
    }
}

impl<T> Eq for Pair<'_, T>
where T: PartialEq + Eq {}

impl<'a, T, Y, Z> Hash for Pair<'a, T>
where T: PartialEq + Eq,
&'a T: IntoIterator<Item = Y>,
Y: BitXor<Output = Z>,
Z: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut merged: Vec<Z> = Vec::new();
        for (a, b) in self.a.into_iter().zip(self.b.into_iter()) {
            merged.push(a.bitxor(b))
        }

        merged.hash(state)
    }
}