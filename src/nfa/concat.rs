use crate::{nfa::NFA, util::Either};

pub struct Concat<L: NFA, R: NFA<A = L::A>>(pub L, pub R);

impl<L: NFA, R: NFA<A = L::A>> NFA for Concat<L, R> {
    type Q = Either<L::Q, R::Q>;
    type A = L::A;

    fn q_init(&self) -> Self::Q {
        Either::Left(self.0.q_init())
    }

    fn q_next(&self, q: &Self::Q, a: Option<&Self::A>) -> Vec<Self::Q> {
        let mut res = vec![];
        match q {
            Either::Left(q) => {
                for nq in self.0.q_next(q, a) {
                    res.push(Either::Left(nq));
                }
                if self.0.is_q_accept(q) && a.is_none() {
                    res.push(Either::Right(self.1.q_init()));
                }
            }
            Either::Right(q) => {
                for nq in self.1.q_next(q, a) {
                    res.push(Either::Right(nq));
                }
            }
        }
        res
    }

    fn is_q_accept(&self, q: &Self::Q) -> bool {
        matches!(q, Either::Right(r) if self.1.is_q_accept(r))
    }
}
