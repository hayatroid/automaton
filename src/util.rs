#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Wrapper<T> {
    Init,
    Q(T),
    Accept,
}
