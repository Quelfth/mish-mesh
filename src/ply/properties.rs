use std::marker::PhantomData;

use plyers::ply::*;
use plyers::ply::PropertyType::*;
use plyers::ply::ScalarType::*;

pub struct PlyPropList<I, V> {
    pub list: Vec<V>,
    phantom: PhantomData<I>
}

impl<I, V> From<Vec<V>> for PlyPropList<I, V> {
    fn from(value: Vec<V>) -> Self {
        Self {list: value, phantom: PhantomData}
    }
}

impl<I, V, const N: usize> From<[V; N]> for PlyPropList<I, V> {
    fn from(value: [V; N]) -> Self {
        Self {list: value.into_iter().collect(), phantom: PhantomData}
    }
}

impl<I, V: Clone> From<&[V]> for PlyPropList<I, V> {
    fn from(value: &[V]) -> Self {
        Self {list: value.to_vec(), phantom: PhantomData}
    }
}


pub trait PlyPropery: Sized {
    const PLY_PROP_TYPE: PropertyType;
    fn into_prop(self) -> Property;
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self>;
}

impl PlyPropery for i8 {
    const PLY_PROP_TYPE: PropertyType = Scalar(Char);
    fn into_prop(self) -> Property { Property::Char(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_char(name) }
}
impl PlyPropery for u8 {
    const PLY_PROP_TYPE: PropertyType = Scalar(UChar);
    fn into_prop(self) -> Property { Property::UChar(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_uchar(name) }
}
impl PlyPropery for i16 {
    const PLY_PROP_TYPE: PropertyType = Scalar(Short);
    fn into_prop(self) -> Property { Property::Short(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_short(name) }
}
impl PlyPropery for u16 {
    const PLY_PROP_TYPE: PropertyType = Scalar(UShort);
    fn into_prop(self) -> Property { Property::UShort(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_ushort(name) }
}
impl PlyPropery for i32 {
    const PLY_PROP_TYPE: PropertyType = Scalar(Int);
    fn into_prop(self) -> Property { Property::Int(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_int(name) }
}
impl PlyPropery for u32 {
    const PLY_PROP_TYPE: PropertyType = Scalar(UInt);
    fn into_prop(self) -> Property { Property::UInt(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_uint(name) }
}
impl PlyPropery for f32 {
    const PLY_PROP_TYPE: PropertyType = Scalar(Float);
    fn into_prop(self) -> Property { Property::Float(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_float(name) }
}
impl PlyPropery for f64 {
    const PLY_PROP_TYPE: PropertyType = Scalar(Double);
    fn into_prop(self) -> Property { Property::Double(self)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_double(name) }
}


//i8
impl PlyPropery for PlyPropList<i8, i8> {
    const PLY_PROP_TYPE: PropertyType = List(Char, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, i8> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, i8> {
    const PLY_PROP_TYPE: PropertyType = List(Short, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, i8> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, i8> {
    const PLY_PROP_TYPE: PropertyType = List(Int, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, i8> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, Char);

    fn into_prop(self) -> Property { Property::ListChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_char(name).map(|l| l.into()) }
}

//u8
impl PlyPropery for PlyPropList<i8, u8> {
    const PLY_PROP_TYPE: PropertyType = List(Char, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, u8> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, u8> {
    const PLY_PROP_TYPE: PropertyType = List(Short, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, u8> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, u8> {
    const PLY_PROP_TYPE: PropertyType = List(Int, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, u8> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, UChar);

    fn into_prop(self) -> Property { Property::ListUChar(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uchar(name).map(|l| l.into()) }
}

//i16
impl PlyPropery for PlyPropList<i8, i16> {
    const PLY_PROP_TYPE: PropertyType = List(Char, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, i16> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, i16> {
    const PLY_PROP_TYPE: PropertyType = List(Short, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, i16> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, i16> {
    const PLY_PROP_TYPE: PropertyType = List(Int, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, i16> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, Short);

    fn into_prop(self) -> Property { Property::ListShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_short(name).map(|l| l.into()) }
}

//u16
impl PlyPropery for PlyPropList<i8, u16> {
    const PLY_PROP_TYPE: PropertyType = List(Char, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, u16> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, u16> {
    const PLY_PROP_TYPE: PropertyType = List(Short, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, u16> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, u16> {
    const PLY_PROP_TYPE: PropertyType = List(Int, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, u16> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, UShort);

    fn into_prop(self) -> Property { Property::ListUShort(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_ushort(name).map(|l| l.into()) }
}

//i32
impl PlyPropery for PlyPropList<i8, i32> {
    const PLY_PROP_TYPE: PropertyType = List(Char, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, i32> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, i32> {
    const PLY_PROP_TYPE: PropertyType = List(Short, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, i32> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, i32> {
    const PLY_PROP_TYPE: PropertyType = List(Int, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, i32> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, Int);

    fn into_prop(self) -> Property { Property::ListInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_int(name).map(|l| l.into()) }
}

//u32
impl PlyPropery for PlyPropList<i8, u32> {
    const PLY_PROP_TYPE: PropertyType = List(Char, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, u32> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, u32> {
    const PLY_PROP_TYPE: PropertyType = List(Short, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, u32> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, u32> {
    const PLY_PROP_TYPE: PropertyType = List(Int, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, u32> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, UInt);

    fn into_prop(self) -> Property { Property::ListUInt(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_uint(name).map(|l| l.into()) }
}

//f32
impl PlyPropery for PlyPropList<i8, f32> {
    const PLY_PROP_TYPE: PropertyType = List(Char, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, f32> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, f32> {
    const PLY_PROP_TYPE: PropertyType = List(Short, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, f32> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, f32> {
    const PLY_PROP_TYPE: PropertyType = List(Int, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, f32> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, Float);

    fn into_prop(self) -> Property { Property::ListFloat(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_float(name).map(|l| l.into()) }
}

//f64
impl PlyPropery for PlyPropList<i8, f64> {
    const PLY_PROP_TYPE: PropertyType = List(Char, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u8, f64> {
    const PLY_PROP_TYPE: PropertyType = List(UChar, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i16, f64> {
    const PLY_PROP_TYPE: PropertyType = List(Short, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u16, f64> {
    const PLY_PROP_TYPE: PropertyType = List(UShort, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<i32, f64> {
    const PLY_PROP_TYPE: PropertyType = List(Int, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}
impl PlyPropery for PlyPropList<u32, f64> {
    const PLY_PROP_TYPE: PropertyType = List(UInt, Double);

    fn into_prop(self) -> Property { Property::ListDouble(self.list)}
    fn from_element_prop<E: PlyElementGet>(element: &E, name: &str) -> Option<Self> { element.get_list_double(name).map(|l| l.into()) }
}