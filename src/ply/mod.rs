use ear_algae::{linear::Vect, traits::Ring, vect};
use plyers::ply::*;
use properties::{PlyPropList, PlyPropery};

use crate::Mesh; 

pub mod properties;


impl<V: Copy+PlyVertex> Mesh<V> {
    pub fn into_ply<E: PlyElementSet>(self, encoding: Encoding) -> Result<Ply<E>, ConsistencyError> {
        let mut ply = Ply::new();
        ply.header.encoding = encoding;
        ply.header.elements.add(V::element_def("vertex".to_owned()));
        ply.header.elements.add({
            let mut def = ElementDef::new("face".to_owned());
            def.properties.add(PropertyDef::new("vertex_index".to_owned(), PlyPropList::<u8, u32>::PLY_PROP_TYPE));
            def
        });
        let verts = self.verts().iter().copied().map(PlyVertex::into_ply_element).collect();
        ply.payload.insert("vertex".to_owned(), verts);
        let faces = self.tris_iter().map(|&t| {
            let mut element = E::new();
            element.set_property("vertex_index".to_owned(), PlyPropList::<u8, _>::from(t.map(|x| x as u32)).into_prop());
            element
        }).collect();
        ply.payload.insert("face".to_owned(), faces);

        ply.make_consistent()?;
        Ok(ply)
    }

    pub fn from_ply<E: PlyElementGet>(ply: Ply<E>) -> Option<Self> {
        let verts = ply.payload.get("vertex")?.iter().map(|v| V::from_ply_element(v)).collect::<Option<Vec<_>>>()?.into_boxed_slice();

        let tris = ply.payload.get("face")?.iter().map(|e| -> Option<_> {
            let list = PlyPropList::<u8, u32>::from_element_prop(e, "vertex_index")?.list;
            if list.len() != 3 {
                return None;
            }
            Some([list[0] as usize, list[1] as usize, list[2] as usize])
        }).collect::<Option<Vec<_>>>()?.into_boxed_slice();

        Some(Mesh {verts, tris})
    }
}



pub trait PlyVertex: Sized {
    fn element_def(name: String) -> ElementDef;
    fn into_ply_element<E: PlyElementSet>(self) -> E;
    fn from_ply_element<E: PlyElementGet>(element: &E) -> Option<Self>;
}





impl<S: Ring + PlyPropery> PlyVertex for Vect<3, S> {

    fn element_def(name: String) -> ElementDef {
        let mut def = ElementDef::new(name);
        def.properties.add(PropertyDef::new("x".to_owned(), S::PLY_PROP_TYPE));
        def.properties.add(PropertyDef::new("y".to_owned(), S::PLY_PROP_TYPE));
        def.properties.add(PropertyDef::new("z".to_owned(), S::PLY_PROP_TYPE));
        
        def
    }

    fn into_ply_element<E: PlyElementSet>(self) -> E {
        let mut vert = E::new();
        vert.set_property("x".to_owned(), self[0].into_prop());
        vert.set_property("y".to_owned(), self[1].into_prop());
        vert.set_property("z".to_owned(), self[2].into_prop());
        vert
    }

    fn from_ply_element<E: PlyElementGet>(element: &E) -> Option<Self> {
        let x = S::from_element_prop(element, "x")?;
        let y = S::from_element_prop(element, "y")?;
        let z = S::from_element_prop(element, "z")?;
        Some(vect!(x, y, z))
    }
}