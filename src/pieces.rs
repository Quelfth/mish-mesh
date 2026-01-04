use std::{fmt::Debug, ops::Index};

use ear_algae::{ops::Aplable, prelude::*};
use index_from_end::{IntoIndex, Len};

mod iter;

#[derive(Copy, Clone, Debug)]
pub struct UMap {
    iter: (usize, usize),
    input: (usize, usize),
    output: (f32, f32),
}

impl UMap {
    pub fn new(iter: (usize, usize), input: (usize, usize), output: (f32, f32)) -> Self {
        Self {
            iter,
            input,
            output,
        }
    }

    pub fn iter(&self) -> impl IntoIterator<Item = usize> {
        self.iter.0..=self.iter.1
    }

    pub fn input_size(&self) -> usize {
        self.input.1 - self.input.0
    }

    pub fn len(&self) -> usize {
        self.iter.1 - self.iter.0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn start_offset(&self) -> isize {
        self.iter.0 as isize - self.input.0 as isize
    }
    pub fn end_offset(&self) -> isize {
        self.input.1 as isize - self.iter.1 as isize
    }

    pub fn map(&self, i: usize) -> f32 {
        if self.input.1 == self.input.0 {
            return 0.5 * (self.output.1 - self.output.0) + self.output.0;
        }
        let zero_one = (i - self.input.0) as f32 / (self.input.1 - self.input.0) as f32;
        zero_one * (self.output.1 - self.output.0) + self.output.0
    }

    pub fn reduce(self, amount: usize) -> Self {
        let iter = (self.iter.0, self.iter.1 - amount);
        let input = (self.input.0, self.input.1 - amount);
        let output = self.output;
        Self {
            iter,
            input,
            output,
        }
    }
}

#[macro_export]
macro_rules! umap {
    (!=$min:expr, !=$max:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max - 1), ($min, $max), (0., 1.))
    };
    (!=$min:expr => $lo:expr, !=$max:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max - 1), ($min, $max), ($lo, 1.))
    };
    (!=$min:expr, !=$max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max - 1), ($min, $max), (0., $hi))
    };
    (!=$min:expr => $lo:expr, !=$max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max - 1), ($min, $max), ($lo, $hi))
    };
    ($min:expr, !=$max:expr) => {
        $crate::pieces::UMap::new(($min, $max - 1), ($min, $max), (0., 1.))
    };
    ($min:expr => $lo:expr, !=$max:expr) => {
        $crate::pieces::UMap::new(($min, $max - 1), ($min, $max), ($lo, 1.))
    };
    ($min:expr, !=$max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min, $max - 1), ($min, $max), (0., $hi))
    };
    ($min:expr => $lo:expr, !=$max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min, $max - 1), ($min, $max), ($lo, $hi))
    };
    (!=$min:expr, $max:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max), ($min, $max), (0., 1.))
    };
    (!=$min:expr => $lo:expr, $max:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max), ($min, $max), ($lo, 1.))
    };
    (!=$min:expr, $max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max), ($min, $max), (0., $hi))
    };
    (!=$min:expr => $lo:expr, $max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min + 1, $max), ($min, $max), ($lo, $hi))
    };
    ($min:expr, $max:expr) => {
        $crate::pieces::UMap::new(($min, $max), ($min, $max), (0., 1.))
    };
    ($min:expr => $lo:expr, $max:expr) => {
        $crate::pieces::UMap::new(($min, $max), ($min, $max), ($lo, 1.))
    };
    ($min:expr, $max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min, $max), ($min, $max), (0., $hi))
    };
    ($min:expr => $lo:expr, $max:expr => $hi:expr) => {
        $crate::pieces::UMap::new(($min, $max), ($min, $max), ($lo, $hi))
    };
}

#[derive(Clone)]
pub struct MeshPatch<V: Copy> {
    verts: Vec<Vec<V>>,
    len_u: usize,
    len_v: usize,
    start_offset_u: isize,
    start_offset_v: isize,
    end_offset_u: isize,
    end_offset_v: isize,
}

impl<V: Copy> Debug for MeshPatch<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeshPatch")
            .field("verts.len()", &self.verts.len())
            .field("len_u", &self.len_u)
            .field("len_v", &self.len_v)
            .field("start_offset_u", &self.start_offset_u)
            .field("start_offset_v", &self.start_offset_v)
            .field("end_offset_u", &self.end_offset_u)
            .field("end_offset_v", &self.end_offset_v)
            .finish()
    }
}

impl<V: Copy> MeshPatch<V> {
    pub fn make(u_map: UMap, v_map: UMap, f: impl Fn(f32, f32) -> V) -> Self {
        let mut verts = Vec::new();

        for i in u_map.iter() {
            let mut row = Vec::new();
            let u = u_map.map(i);
            for j in v_map.iter() {
                let v = v_map.map(j);
                row.push(f(u, v));
            }
            verts.push(row);
        }

        if verts.is_empty() {
            panic!("Attempted to create a mesh patch with no verts!")
        }

        //println!("{} :: {}({}) / {}({})", verts.len(), u_map.input_size(), u_map.len(), v_map.input_size(), v_map.len());
        Self {
            verts,
            len_u: u_map.input_size(),
            len_v: v_map.input_size(),
            start_offset_u: u_map.start_offset(),
            end_offset_u: u_map.end_offset(),
            start_offset_v: v_map.start_offset(),
            end_offset_v: v_map.end_offset(),
        }
    }

    pub fn u_start<'a>(
        &'a self,
    ) -> VirtualMeshLinearSegment<'a, V, impl Fn(usize) -> &'a V + Clone> {
        VirtualMeshLinearSegment::new(self.start_offset_v, self.end_offset_v, self.len_v, |i| {
            &self.verts[0][self.map_j(i)]
        })
    }
    pub fn u_end<'a>(&'a self) -> VirtualMeshLinearSegment<'a, V, impl Fn(usize) -> &'a V + Clone> {
        VirtualMeshLinearSegment::new(self.start_offset_v, self.end_offset_v, self.len_v, |i| {
            &self.verts[Len - 1][self.map_j(i)]
        })
    }
    pub fn v_start<'a>(
        &'a self,
    ) -> VirtualMeshLinearSegment<'a, V, impl Fn(usize) -> &'a V + Clone> {
        VirtualMeshLinearSegment::new(self.start_offset_u, self.end_offset_u, self.len_u, |i| {
            &self.verts[self.map_i(i)][0]
        })
    }
    pub fn v_end<'a>(&'a self) -> VirtualMeshLinearSegment<'a, V, impl Fn(usize) -> &'a V + Clone> {
        VirtualMeshLinearSegment::new(self.start_offset_u, self.end_offset_u, self.len_u, |i| {
            &self.verts[self.map_i(i)][Len - 1]
        })
    }
    pub fn quad(&self, i: usize, j: usize) -> [V; 4] {
        [
            self.verts[i][j],
            self.verts[i + 1][j],
            self.verts[i + 1][j + 1],
            self.verts[i][j + 1],
        ]
    }

    fn map_i(&self, i: usize) -> usize {
        (i as isize - self.start_offset_u) as usize
    }

    fn map_j(&self, j: usize) -> usize {
        (j as isize - self.start_offset_v) as usize
    }

    pub fn vert(&self, i: impl IntoIndex<usize>, j: impl IntoIndex<usize>) -> &V {
        let i = i.into_index(self.len_u);
        let j = j.into_index(self.len_v);

        //println!("{} : {} ~~~ {}, {}", self.verts.len(), self.len_u, i, j);
        &self.verts[self.map_i(i)][self.map_j(j)]
    }

    pub fn len_u(&self) -> usize {
        self.verts.len()
    }
    pub fn len_v(&self) -> usize {
        self.verts[0].len()
    }

    pub fn flip(&self) -> Flipped<&Self> {
        Flipped(self)
    }
}

impl<V: Copy, A: Apl<V> + Copy> Aplable<A> for &MeshPatch<V>
where
    <A as Apl<V>>::Output: Copy,
{
    type Output = MeshPatch<<A as Apl<V>>::Output>;

    fn apply(self, apler: A) -> Self::Output {
        let &MeshPatch {
            verts: _,
            len_u,
            len_v,
            start_offset_u,
            start_offset_v,
            end_offset_u,
            end_offset_v,
        } = self;
        MeshPatch {
            verts: self
                .verts
                .iter()
                .map(|row| row.iter().map(|&v| apler.apl(v)).collect())
                .collect(),
            len_u,
            len_v,
            start_offset_u,
            start_offset_v,
            end_offset_u,
            end_offset_v,
        }
    }
}

#[derive(Clone)]
pub struct MeshTriPatch<V: Copy> {
    verts: Vec<Vec<V>>,
}

impl<V: Copy> MeshTriPatch<V> {
    pub fn make_polar(r_map: UMap, theta_map: UMap, f: impl Fn(f32, f32) -> V) -> Self {
        let mut verts = Vec::new();
        for (n, i) in r_map.iter().into_iter().enumerate() {
            let mut row = Vec::new();
            let r = r_map.map(i);
            let theta_map = theta_map.reduce(n);
            for j in theta_map.iter() {
                let theta = theta_map.map(j);
                row.push(f(r, theta))
            }
            verts.push(row)
        }
        Self { verts }
    }

    pub fn r_start(&self) -> impl DoubleEndedIterator<Item = &V> + Clone {
        self.verts[0].iter()
    }

    pub fn r_end(&self) -> &V {
        &self.verts[Len - 1][0]
    }

    pub fn theta_start(&self) -> impl DoubleEndedIterator<Item = &V> + Clone {
        self.verts.iter().map(|row| &row[0])
    }

    pub fn theta_end(&self) -> impl DoubleEndedIterator<Item = &V> + Clone {
        self.verts.iter().map(|row| &row[Len - 1])
    }

    pub fn flip(&self) -> Flipped<&Self> {
        Flipped(self)
    }
}

#[derive(Clone)]
pub struct MeshStrand<V: Copy> {
    verts: Vec<V>,
    len: usize,
    start_offset: isize,
    end_offset: isize,
}

impl<V: Copy> MeshStrand<V> {
    pub fn make(us: UMap, f: impl Fn(f32) -> V) -> Self {
        let mut verts = Vec::new();

        for i in us.iter() {
            let u = us.map(i);
            verts.push(f(u))
        }

        if verts.is_empty() {
            panic!("Attempted to create a mesh strand with no verts!")
        }

        Self {
            verts,
            len: us.input_size(),
            start_offset: us.start_offset(),
            end_offset: us.end_offset(),
        }
    }

    pub fn start(&self) -> V {
        self.verts[0]
    }

    pub fn end(&self) -> V {
        self.verts[Len - 1]
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &V> + Clone {
        self.verts.iter()
    }

    pub fn seam<'a>(&'a self) -> VirtualMeshLinearSegment<'a, V, impl Fn(usize) -> &'a V + Clone> {
        VirtualMeshLinearSegment::new(self.start_offset, self.end_offset, self.len, |i| {
            &self.verts[self.map_i(i)]
        })
    }

    fn map_i(&self, i: usize) -> usize {
        (i as isize - self.start_offset) as usize
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.verts.len()
    }
}

impl<I, V: Copy> Index<I> for MeshStrand<V>
where
    Vec<V>: Index<I, Output = V>,
{
    type Output = V;

    fn index(&self, index: I) -> &Self::Output {
        self.verts.index(index)
    }
}

impl<V: Copy, A: Apl<V> + Copy> Aplable<A> for &MeshStrand<V>
where
    <A as Apl<V>>::Output: Copy,
{
    type Output = MeshStrand<<A as Apl<V>>::Output>;

    fn apply(self, apler: A) -> Self::Output {
        let &MeshStrand {
            verts: _,
            len,
            start_offset,
            end_offset,
        } = self;
        MeshStrand {
            verts: self.verts.iter().map(|&v| apler.apl(v)).collect(),
            len,
            start_offset,
            end_offset,
        }
    }
}

pub struct VirtualMeshLinearSegment<'v, V: Copy + 'v, M: Fn(usize) -> &'v V> {
    start_offset: isize,
    end_offset: isize,
    len: usize,
    map: M,
}

impl<'v, V: Copy + 'v, M: Fn(usize) -> &'v V> VirtualMeshLinearSegment<'v, V, M> {
    fn new(start_offset: isize, end_offset: isize, len: usize, map: M) -> Self {
        Self {
            start_offset,
            end_offset,
            map,
            len,
        }
    }

    pub fn vert(&self, index: impl IntoIndex<usize>) -> &'v V {
        (self.map)(index.into_index(self.len))
    }

    pub fn verts(
        &self,
        start: impl IntoIndex<usize>,
        end: impl IntoIndex<usize>,
    ) -> std::iter::Map<std::ops::Range<usize>, &M> {
        let range = start.into_index(self.len)..end.into_index(self.len);
        range.into_iter().map(&self.map)
    }

    pub fn verts1(
        &self,
        index: impl IntoIndex<usize>,
    ) -> std::iter::Map<std::ops::Range<usize>, &M> {
        let index = index.into_index(self.len);
        let range = index..index + 1;
        range.into_iter().map(&self.map)
    }

    pub fn all(&self) -> std::iter::Map<std::ops::Range<usize>, &M> {
        self.verts(
            self.start_offset as usize,
            (self.len as isize + 1 - self.end_offset) as usize,
        )
    }
}

pub struct Flipped<T: IntoIterator>(pub(crate) T);
