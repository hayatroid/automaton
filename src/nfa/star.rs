use crate::{nfa::NFA, util::Wrapper};

pub struct Star<N>(pub N);

impl<N: NFA> NFA for Star<N> {
    type Q = Wrapper<N::Q>;
    type A = N::A;

    fn q_init(&self) -> Self::Q {
        Wrapper::Init
    }

    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q> {
        let mut res = vec![];
        match q {
            Wrapper::Init => {
                if a.is_none() {
                    res.push(Wrapper::Q(self.0.q_init()));
                    res.push(Wrapper::Accept);
                }
            }
            Wrapper::Q(q) => {
                for nq in self.0.q_next(q, a) {
                    res.push(Wrapper::Q(nq));
                }
                if self.0.is_q_accept(q) && a.is_none() {
                    res.push(Wrapper::Q(self.0.q_init()));
                    res.push(Wrapper::Accept);
                }
            }
            Wrapper::Accept => (),
        }
        res
    }

    fn is_q_accept(&self, q: &Self::Q) -> bool {
        matches!(q, Wrapper::Accept)
    }
}
