use ndarray::prelude::*;
use ndarray::{Data, RemoveAxis, Zip};
use num::{traits::FloatConst, Float, NumCast, Zero};
use std::cmp::Ordering;

pub trait DebugMinMax<T> {
    fn debug_min(&self) -> T;
    fn debug_max(&self) -> T;
}

impl<T> DebugMinMax<T> for Vec<T>
where
    T: Float + FloatConst,
{
    fn debug_min(&self) -> T {
        self.iter().fold(T::infinity(), |a: T, &b| a.min(b))
    }

    fn debug_max(&self) -> T {
        self.iter().fold(T::neg_infinity(), |a: T, &b| a.max(b))
    }
}

// impl<f32> DebugMinMax<f32> for Vec<f32> {}
// impl<f64> DebugMinMax<f64> for Vec<f64> {}

// Type invariant: Each index appears exactly once
#[derive(Clone, Debug)]
pub struct Permutation {
    indices: Vec<usize>,
}

impl Permutation {
    pub fn from_indices(v: Vec<usize>) -> Result<Self, ()> {
        let perm = Permutation { indices: v };
        if perm.correct() {
            Ok(perm)
        } else {
            Err(())
        }
    }

    fn correct(&self) -> bool {
        let axis_len = self.indices.len();
        let mut seen = vec![false; axis_len];
        for &i in &self.indices {
            match seen.get_mut(i) {
                None => return false,
                Some(s) => {
                    if *s {
                        return false;
                    } else {
                        *s = true;
                    }
                }
            }
        }
        true
    }
}

pub trait SortArray {
    fn identity(&self, axis: Axis) -> Permutation;
    fn sort_axis_by<F>(&self, axis: Axis, less_than: F) -> Permutation
    where
        F: FnMut(usize, usize) -> bool;
}

impl<A, S, D> SortArray for ArrayBase<S, D>
where
    S: Data<Elem = A>,
    D: Dimension,
{
    fn identity(&self, axis: Axis) -> Permutation {
        Permutation {
            indices: (0..self.len_of(axis)).collect(),
        }
    }

    fn sort_axis_by<F>(&self, axis: Axis, mut less_than: F) -> Permutation
    where
        F: FnMut(usize, usize) -> bool,
    {
        let mut perm = self.identity(axis);
        perm.indices.sort_by(move |&a, &b| {
            if less_than(a, b) {
                Ordering::Less
            } else if less_than(b, a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        perm
    }
}
