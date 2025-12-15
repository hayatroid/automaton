pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub enum Wrapper<T> {
    Init,
    Q(T),
    Accept,
}
