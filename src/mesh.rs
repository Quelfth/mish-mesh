use std::{collections::HashSet, iter, slice, sync::Mutex};

use ear_algae::{prelude::*, traits::{Field, Ring}};

use crate::{vertex::{VertexNormal, VertexPosition}, Mesh, Tri};

use rayon::prelude::*;

impl<V: Copy> Mesh<V> {
    pub fn verts(&self) -> &[V] {
        &self.verts
    }
    
    pub fn verts_mut(&mut self) -> &mut [V] {
        &mut self.verts
    }

    pub fn tris_iter(&self) -> slice::Iter<'_, [usize; 3]> {
        self.tris.iter()
    }

    pub fn tris_iter_flat(&self) -> iter::Flatten<slice::Iter<'_, [usize; 3]>>  {
        self.tris.iter().flatten()
    }

    pub fn tris_flat(&self) -> Vec<usize> {
        self.tris_iter_flat().copied().collect()
    }

    pub fn tris_flat_mapped<T>(&self, f: impl Fn(usize) -> T) -> Vec<T> {
        self.tris_iter_flat().map(|&i| f(i)).collect()
    }

    pub fn tri_verts(&self) -> Vec<Tri<V>> {
        self.tris.iter().map(|&[a, b, c]| Tri::new(self.verts[a], self.verts[b], self.verts[c])).collect::<Vec<_>>()
    }

    pub fn map<T: Copy>(&self, f: impl Fn(V) -> T) -> Mesh<T> {
        let verts = self.verts.iter().map(|&x| f(x)).collect::<Vec<_>>().into_boxed_slice();
        Mesh {verts, tris: self.tris.clone()}
    }

    


    pub fn autocomplete_normals_sync(self) -> Self where V: VertexPosition + VertexNormal {
        let Mesh {mut verts, tris} = self;
        let mut existing = HashSet::new();

        for (i, vert) in verts.iter().enumerate() {
            if vert.normal() != Vect::ZERO {
                existing.insert(i);
            } 
        }

        for tri in tris.iter() {
            let a = verts[tri[0]].pos();
            let b = verts[tri[1]].pos();
            let c = verts[tri[2]].pos();
            if let Some(normal) = (b-a).cross(c-a).normal() {
                for i in 0..2 {
                    if !existing.contains(&tri[i]) {
                        let a = verts[tri[i]].pos();
                        let b = verts[tri[(i+1)%3]].pos();
                        let c = verts[tri[(i+2)%3]].pos();
                        let e0 = b-a;
                        let e1 = c-a;
                        let angle = e0.dot(e1).div(e0.magn().mul(e1.magn())).acos();
                        verts[tri[i]].set_normal(verts[tri[i]].normal() + normal.relax() * angle);
                    }
                }
            }
        }

        for vert in verts.iter_mut() {
            vert.set_normal(vert.normal().normal_or_zero());
        }

        Mesh {verts, tris}
    }

    pub fn autocomplete_normals(self) -> Self where V: VertexPosition + VertexNormal + Send + Sync, V::Scalar: Send + Sync {
        
        let Mesh {mut verts, tris} = self;

        let existing = verts.par_iter().enumerate().fold(HashSet::new, |mut set, (i, vert)| {
            if vert.normal() != Vect::ZERO {
                set.insert(i);
            }
            set
        }).reduce(HashSet::new, |mut c, n| {c.extend(n); c});

        let normals = {
            let mut normals = Vec::new();
            verts.par_iter().map(|v| Mutex::new(v.normal())).collect_into_vec(&mut normals);
            normals
        };

        tris.par_iter().for_each(|tri| {
            let a = verts[tri[0]].pos();
            let b = verts[tri[1]].pos();
            let c = verts[tri[2]].pos();
            if let Some(normal) = (b-a).cross(c-a).normal() {
                for i in 0..2 {
                    if !existing.contains(&tri[i]) {
                        if let Ok(mut lock) = normals[tri[i]].lock() {
                            let a = verts[tri[i]].pos();
                            let b = verts[tri[(i+1)%3]].pos();
                            let c = verts[tri[(i+2)%3]].pos();
                            let e0 = b-a;
                            let e1 = c-a;
                            let angle = e0.dot(e1).div(e0.magn().mul(e1.magn())).acos();
                            *lock += normal * angle;
                        } else {
                            println!("Mutex error during normal computation.")
                        }
                    }
                }
            }
        });

        verts.par_iter_mut().zip(normals.into_par_iter().map(Mutex::into_inner)).for_each(|(vert, normal)| {
            if let Ok(normal) = normal {
                vert.set_normal(normal.normal_or_zero())
            } else {
                eprintln!("Mutex error during final normal setting.")
            }
        });

        Mesh {verts, tris}
    }
    
}


