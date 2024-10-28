use std::array;

use ear_algae::prelude::*;


use crate::vertex::VertexPosition;

use super::{Seg, Tri};


impl<V: Copy> Tri<V> {
    pub fn new(a: V, b: V, c: V) -> Self {
        Self(a, b, c)
    }

    pub fn map_out<U>(&self, f: impl Fn(V) -> U) -> [U; 3] {
        [self.0, self.1, self.2].map(f)
    }

    pub fn map<U: Copy>(&self, f: impl Fn(V) -> U) -> Tri<U> {
        Tri(f(self.0), f(self.1), f(self.2))
    }

    pub fn edge_iter(self) -> array::IntoIter<Seg<V>, 3> {
        [
            Seg(self.0, self.1),
            Seg(self.1, self.2),
            Seg(self.2, self.0)
        ].into_iter()
    }
}
impl<V: Copy> IntoIterator for Tri<V> {
    type Item = V;

    type IntoIter = array::IntoIter<V, 3>;

    fn into_iter(self) -> array::IntoIter<V, 3>  {
        [self.0, self.1, self.2].into_iter()
    }
}

impl<V: VertexPosition> Tri<V> {
    pub fn normal(self) -> Option<Nrml<3, V::Scalar>> {
        (self.1.pos()-self.0.pos()).cross(self.2.pos()-self.0.pos()).normal()
    }
} 