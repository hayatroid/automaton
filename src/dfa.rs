pub mod subset;

pub trait DFA {
    type Q;
    type A;
    fn q_init(&self) -> Self::Q;
    fn q_next(&self, q: &Self::Q, a: &Self::A) -> Self::Q;
    fn is_q_accept(&self, q: &Self::Q) -> bool;
}
