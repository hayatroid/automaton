use std::collections::BTreeSet;

use crate::{dfa::DFA, nfa::NFA};

pub struct Subset<N: NFA>(pub N);

impl<N: NFA<Q: Clone + Ord>> DFA for Subset<N> {
    type Q = BTreeSet<N::Q>;
    type A = N::A;

    fn q_init(&self) -> Self::Q {
        let res = BTreeSet::from([self.0.q_init()]);
        self.eps_closure(res)
    }

    fn q_next(&self, q: &Self::Q, a: &Self::A) -> Self::Q {
        let res = q.iter().flat_map(|q| self.0.q_next(q, Some(a))).collect();
        self.eps_closure(res)
    }

    fn is_q_accept(&self, q: &Self::Q) -> bool {
        q.iter().any(|q| self.0.is_q_accept(q))
    }
}

impl<N: NFA<Q: Clone + Ord>> Subset<N> {
    fn eps_closure(&self, mut res: BTreeSet<N::Q>) -> BTreeSet<N::Q> {
        let mut dfs: Vec<N::Q> = res.iter().cloned().collect();
        while let Some(q) = dfs.pop() {
            for nq in self.0.q_next(&q, None) {
                res.insert(nq.clone()).then(|| dfs.push(nq));
            }
        }
        res
    }
}
