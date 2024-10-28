

use std::collections::HashMap;

use crate::vertex::VertexPosition;

use super::Mesh;

use by_address::ByAddress;

#[derive(Default)]
pub struct MeshBuilder<'v, V: Copy> {
    pub tris: Vec<[&'v V; 3]>
}


impl<'v, V: Copy> MeshBuilder<'v, V> {

    pub fn new() -> Self {
        MeshBuilder {
            tris: Vec::new()
        }
    }

    pub fn add_poly<const N: usize>(&mut self, poly: [&'v V; N]) {
        for i in 1..N-1 {
            self.tris.push([poly[0], poly[i], poly[i+1]])
        }
    }

    pub fn add_quad(&mut self, quad: [&'v V; 4]) where V: VertexPosition {
        let a = (quad[0].pos() - quad[2].pos()).sqr_magn();
        let b = (quad[1].pos() - quad[3].pos()).sqr_magn();
        if a < b {
            self.tris.push([quad[0], quad[1], quad[2]]);
            self.tris.push([quad[0], quad[2], quad[3]]);
        } else {
            self.tris.push([quad[1], quad[2], quad[3]]);
            self.tris.push([quad[1], quad[3], quad[0]]);
        }
    }

    pub fn patch<const N: usize>(&mut self, patch: impl IntoIterator<Item = [&'v V; N]> ) {
        for poly in patch {
            self.add_poly(poly)
        }
    }

    pub fn patch_quads(&mut self, patch: impl IntoIterator<Item = [&'v V; 4]> ) where V: VertexPosition {
        for quad in patch {
            self.add_quad(quad)
        }
    }

    pub fn stitch(&mut self, mut left: impl Iterator<Item = &'v V>, mut right: impl Iterator<Item = &'v V>) where V: VertexPosition {
        let mut l0 = if let Some(l) = left.next() {l} else {return};
        let mut r0 = if let Some(r) = right.next() {r} else {return};
        for (l1, r1) in left.zip(right) {
            self.add_quad([l0, r0, r1, l1]);
            (l0, r0) = (l1, r1)
        }
    }

    pub fn stitch_loop(&mut self, mut left: impl Iterator<Item = &'v V>, mut right: impl Iterator<Item = &'v V>) where V: VertexPosition {
        let l0 = if let Some(l) = left.next() {l} else {return};
        let r0 = if let Some(r) = right.next() {r} else {return};
        let mut l1 = l0;
        let mut r1 = r0;
        for (l2, r2) in left.zip(right) {
            self.add_quad([l1, r1, r2, l2]);
            (l1, r1) = (l2, r2)
        }

        self.add_quad([l1, r1, r0, l0]);
    }

    pub fn sinch_loop(&mut self, mut seam: impl Iterator<Item = &'v V>, knot: &'v V) {
        let v0 = if let Some(v) = seam.next() {v} else {return};
        
        let mut v1 = v0;
        for v2 in seam {
            self.add_poly([v1, v2, knot]);
            v1 = v2;
        }
        
        self.add_poly([v1, v0, knot])
    }

    // pub fn finish(self, merge_threshold: V::Scalar) -> super::Mesh<V> where V: Sync, V::Scalar: Sync {
    //     let time = Instant::now();

    //     let mut verts: Vec<V> = Vec::new();
    //     let mut tris = Vec::new();

    //     let mut map_closure = |v: V| {
    //         match verts.par_iter().enumerate().find_any(|&(_, &e)| e.close_to(v, merge_threshold)) {
    //             None => {
    //                 let i = verts.len();
    //                 verts.push(v);
    //                 i
    //             }
    //             Some((i, _)) => i
    //         } 
    //     };
        
    //     let n = self.tris.len();

    //     let mut progress_bar = 0;

    //     print!("[                    ]");

    //     _ = stdout().flush();

    //     for (i, tri) in self.tris.into_iter().enumerate() {
    //         tris.push(tri.map(&mut map_closure));
    //         if i*20/n > progress_bar {
    //             progress_bar += 1;
    //             let mut bar = "\r[".to_owned();
    //             for j in 0..20 {
    //                 bar += match j.cmp(&progress_bar) {
    //                     Ordering::Less => "=",
    //                     Ordering::Equal => ">",
    //                     Ordering::Greater => " ",
    //                 };
    //             }
    //             bar += "]";
    //             print!("{}", bar);
    //             _ = stdout().flush();
    //         }
    //     }   
        
    //     let verts = verts.into_boxed_slice();
    //     let tris = tris.into_boxed_slice();

    //     println!("\rMesh finish took {}s", time.elapsed().as_secs_f32());
    //     Mesh {verts, tris}
    // }

    

    // pub fn finish_sync(self, merge_threshold: V::Scalar) -> super::Mesh<V> {
    //     let time = Instant::now();

    //     let mut verts: Vec<V> = Vec::new();
    //     let mut indices = Vec::new();

    //     let mut map_closure = |v: V| {
            
    //         match verts.iter().enumerate().find(|&(_, &e)| e.close_to(v, merge_threshold)) {
    //             None => {
    //                 let i = verts.len();
    //                 verts.push(v);
    //                 i
    //             }
    //             Some((i, _)) => i
    //         } 
    //     };
    //     for tri in self.tris.into_iter() {
    //         indices.push(tri.map(&mut map_closure));
    //     }   
        
    //     let verts = verts.into_boxed_slice();
    //     let indices = indices.into_boxed_slice();

    //     println!("Mesh finish took {}s", time.elapsed().as_secs_f32());
    //     Mesh {verts, tris: indices}
    // }

    pub fn finish(self) -> super::Mesh<V> {

        let mut map: HashMap<ByAddress<&'v V>, usize> = HashMap::new();
        let mut verts = Vec::new();
        let mut tris = Vec::new();
        
        for tri in self.tris.into_iter() {
            tris.push(tri.map(|v: &'v V| {

                let addr = ByAddress(v);
                
                match map.get(&addr) {
                    None => {
                        let i = verts.len();
                        map.insert(addr, i);
                        verts.push(*v);
                        i
                    }
                    Some(&i) => i
                } 
            }));
        }   
        
        let verts = verts.into_boxed_slice();
        let tris = tris.into_boxed_slice();

        Mesh {verts, tris}
    }


}


