use ear_algae::{traits::Field, Vect};

// pub trait VertexMerge: VertexData {
//     fn close_to(self, other: Self, threshold: Self::Scalar) -> bool;
//     fn merge(self, other: Self) -> Self;
// }

// impl<S: Field> VertexMerge for Vect<3, S> {
//     fn close_to(self, other: Self, threshold: Self::Scalar) -> bool {
//         (self-other).sqr_magn() < threshold.pow(2)
//     }

//     fn merge(self, other: Self) -> Self {
//         todo!()
//     }
// }

pub trait VertexData: Copy {
    type Scalar: Field;
}

pub trait VertexPosition: VertexData {
    fn pos(self) -> Vect<3, Self::Scalar>;
    fn set_pos(&mut self, pos: Vect<3, Self::Scalar>);
}

pub trait VertexNormal: VertexData {
    fn normal(self) -> Vect<3, Self::Scalar>;
    fn set_normal(&mut self, normal: Vect<3, Self::Scalar>);
}

impl<S: Field> VertexData for Vect<3, S> {
    type Scalar = S;
}

impl<S: Field> VertexPosition for Vect<3, S> {
    fn pos(self) -> Vect<3, Self::Scalar> {
        self
    }

    fn set_pos(&mut self, pos: Vect<3, Self::Scalar>) {
        *self = pos
    }
}
