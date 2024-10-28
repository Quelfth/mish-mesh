use serde::{Deserialize, Serialize};


pub mod mesh_builder;
pub mod pieces;
pub mod tri;
pub mod vertex;
pub mod mesh;
pub mod ply;
pub mod merge_by_distance;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mesh<V: Copy> {
    verts: Box<[V]>,
    tris: Box<[[usize; 3]]>
}



#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tri<V: Copy>(pub V, pub V, pub V);



pub struct Seg<V: Copy>(pub V, pub V);
