use std::{iter::*, slice::Iter};


use index_from_end::{IntoIndex, Len};

use crate::pieces::MeshPatch;

use super::{Flipped, MeshTriPatch, VirtualMeshLinearSegment};



impl<'a, V: Copy> IntoIterator for &'a MeshPatch<V> {
    type Item = [&'a V; 4];

    type IntoIter = Map<Zip<Zip<FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>, FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>>, Zip<FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>, FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>>>, fn(((&'a V, &'a V), (&'a V, &'a V))) -> [&'a V; 4]>;

    fn into_iter(self) -> Self::IntoIter  {
        let a = self.verts[..self.verts.len()-1].iter().flat_map((|v| v[..v.len()-1].iter()) as fn(&Vec<V>) -> Iter<V>);

        let b = self.verts[1..].iter().flat_map((|v| v[..v.len()-1].iter()) as fn(&Vec<V>) -> Iter<V>);
        let c = self.verts[1..].iter().flat_map((|v| v[1..].iter()) as fn(&Vec<V>) -> Iter<V>);
        let d = self.verts[..self.verts.len()-1].iter().flat_map((|v| v[1..].iter()) as fn(&Vec<V>) -> Iter<V>);
        
        a.zip(b).zip(c.zip(d)).map(|((a, b), (c, d))| [a, b, c, d])
    }
}




impl<'a, V: Copy> IntoIterator for &'a MeshTriPatch<V> {
    type Item = [&'a V; 3];

    type IntoIter = Chain<Map<Zip<Zip<Flatten<Skip<Iter<'a, Vec<V>>>>, FlatMap<Iter<'a, Vec<V>>, Skip<Iter<'a, V>>, fn(&'a Vec<V>) -> Skip<Iter<'a, V>>>>, FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>>, fn(((&'a V, &'a V), &'a V)) -> [&'a V; 3]>, Map<Zip<Zip<FlatMap<Iter<'a, Vec<V>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>, FlatMap<Skip<Iter<'a, Vec<V>>>, Iter<'a, V>, fn(&'a Vec<V>) -> Iter<'a, V>>>, FlatMap<Skip<Iter<'a, Vec<V>>>, Skip<Iter<'a, V>>, fn(&'a Vec<V>) -> Skip<Iter<'a, V>>>>, fn(((&'a V, &'a V), &'a V)) -> [&'a V; 3]>>;

    fn into_iter(self) -> Self::IntoIter {
        fn checked_range<V>(v: &[V], min: impl IntoIndex<usize>, max: impl IntoIndex<usize>) -> &[V] {
            let min = min.into_index(v.len());
            let max = max.into_index(v.len());
            if min <= max {
                &v[min..max]
            } else {
                &[]
            }
        }

        let a = self.verts.iter().skip(1).flatten();
        let b = self.verts.iter().flat_map((|v| v.iter().skip(1)) as fn(&Vec<V>) -> Skip<Iter<V>>);
        let c = self.verts.iter().flat_map((|v| v[..v.len()-1].iter()) as fn(&Vec<V>) -> Iter<V>);
        let part1 = a.zip(b).zip(c).map((|((a,b),c)| [a, b, c]) as fn(((&'a V, &'a V), &'a V)) -> [&'a V; 3]);

        let d = self.verts[..self.verts.len()-1].iter().flat_map((|v| checked_range(v, 1, Len-1).iter()) as fn(&Vec<V>) -> Iter<V>);
        let e = self.verts.iter().skip(1).flat_map((|v| v[..v.len()-1].iter()) as fn(&Vec<V>) -> Iter<V>);
        let f = self.verts.iter().skip(1).flat_map((|v| v.iter().skip(1)) as fn(&Vec<V>) -> Skip<Iter<V>>);
        let part2 = d.zip(e).zip(f).map((|((d, e), f)| [d, e, f]) as fn(((&'a V, &'a V), &'a V)) -> [&'a V; 3]);

        part1.chain(part2)
    }
}

impl<V, const N: usize, T: IntoIterator<Item = [V; N]>> IntoIterator for Flipped<T> {
    type Item = [V; N];

    type IntoIter = Map<T::IntoIter, fn([V; N]) -> [V; N]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|mut p| {p.reverse(); p})
    }
}

impl<'a, 'v, V: Copy, M: Fn(usize) -> &'v V + Copy> IntoIterator for &'a VirtualMeshLinearSegment<'v, V, M> {
    type Item = &'v V;

    type IntoIter = Map<std::ops::Range<usize>, &'a M>;

    fn into_iter(self) -> Self::IntoIter {
        self.all()
    }
}