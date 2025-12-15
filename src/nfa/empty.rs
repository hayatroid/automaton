use std::marker::PhantomData;

use crate::{nfa::NFA, util::Wrapper};

pub struct Empty<A>(pub PhantomData<A>);

impl<A> NFA for Empty<A> {
    type Q = Wrapper<()>;
    type A = A;

    fn q_init(&self) -> Self::Q {
        Wrapper::Init
    }

    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q> {
        let mut res = vec![];
        if matches!(q, Wrapper::Init) && a.is_none() {
            res.push(Wrapper::Accept);
        }
        res
    }

    fn is_q_accept(&self, q: &Self::Q) -> bool {
        matches!(q, Wrapper::Accept)
    }
}
