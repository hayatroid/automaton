use crate::{nfa::NFA, util::Wrapper};

pub struct Symbol<A: PartialEq>(pub A);

impl<A: PartialEq> NFA for Symbol<A> {
    type Q = Wrapper<()>;
    type A = A;

    fn q_init(&self) -> Self::Q {
        Wrapper::Init
    }

    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q> {
        let mut res = vec![];
        if matches!(q, Wrapper::Init) && matches!(a, Some(a) if a == &self.0) {
            res.push(Wrapper::Accept);
        }
        res
    }

    fn is_q_accept(&self, q: &Self::Q) -> bool {
        matches!(q, Wrapper::Accept)
    }
}
