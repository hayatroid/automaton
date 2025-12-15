use crate::{
    nfa::NFA,
    util::{Either, Wrapper},
};

pub struct Union<L: NFA, R: NFA<A = L::A>>(pub L, pub R);

impl<L: NFA, R: NFA<A = L::A>> NFA for Union<L, R> {
    type Q = Wrapper<Either<L::Q, R::Q>>;
    type A = L::A;

    fn q_init(&self) -> Self::Q {
        Wrapper::Init
    }

    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q> {
        let mut res = vec![];
        match q {
            Wrapper::Init => {
                if a.is_none() {
                    res.push(Wrapper::Q(Either::Left(self.0.q_init())));
                    res.push(Wrapper::Q(Either::Right(self.1.q_init())));
                }
            }
            Wrapper::Q(Either::Left(q)) => {
                for nq in self.0.q_next(q, a) {
                    res.push(Wrapper::Q(Either::Left(nq)));
                }
                if self.0.is_q_accept(q) && a.is_none() {
                    res.push(Wrapper::Accept);
                }
            }
            Wrapper::Q(Either::Right(q)) => {
                for nq in self.1.q_next(q, a) {
                    res.push(Wrapper::Q(Either::Right(nq)));
                }
                if self.1.is_q_accept(q) && a.is_none() {
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
