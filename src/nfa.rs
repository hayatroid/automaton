pub mod concat;
pub mod union;

pub trait NFA {
    type Q;
    type A;
    fn q_init(&self) -> Self::Q;
    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q>;
    fn is_q_accept(&self, q: &Self::Q) -> bool;
}
