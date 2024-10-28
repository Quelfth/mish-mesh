use std::collections::HashMap;

use ear_algae::traits::Ring;

use kiddo::{float::kdtree::Axis, float::kdtree::KdTree, NearestNeighbour, SquaredEuclidean};

use crate::{vertex::{VertexData, VertexPosition}, Mesh};




impl<V: VertexPosition> Mesh<V> where <V as VertexData>::Scalar : Axis {
    pub fn merge_by_distance(self, distance: V::Scalar) -> Self {
        
        let mut tree: KdTree<V::Scalar, usize, 3, 256, u32> = KdTree::new();
        let mut tree_verts: HashMap<usize, V> = HashMap::new();

        for (i, v) in self.verts().iter().enumerate() {
            tree.add(&v.pos().as_array(), i);
            tree_verts.insert(i, *v);
        }

        let mut index_map = HashMap::<usize, usize>::new();
        let mut verts = Vec::new();

        let mut i = 0;
        while !tree_verts.is_empty() {
            while !tree_verts.contains_key(&i) {i += 1}

            
            let v = tree_verts.remove(&i).unwrap(); // We just checked this above.
            let j = verts.len();
            verts.push(v);
            index_map.insert(i, j);

            for NearestNeighbour{item: i, ..} in tree.within_unsorted::<SquaredEuclidean>(&v.pos().as_array(), distance.pow(2)) {
                if tree_verts.remove(&i).is_some() {
                    index_map.insert(i, j);
                }
            }
        }
        
        let verts = verts.into_boxed_slice();
        let tris = self.tris_iter().map(|t| t.map(|i| index_map[&i])).filter(|t| t[0] != t[1] && t[1] != t[2] && t[2] != t[0]).collect::<Vec<_>>().into_boxed_slice();

        Mesh {
            verts,
            tris
        }
    }
}