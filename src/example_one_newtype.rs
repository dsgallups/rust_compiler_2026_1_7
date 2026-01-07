pub trait TestTrait {
    type A;
    type B;

    fn test_traverse<P>(self, processor: P)
    where
        P: PassedIn<Self::A>,
        P::Visited: PassedIn<Self::B>;
}

struct Ab<A, B>((A, B));

impl<A, B> TestTrait for Ab<A, B> {
    type A = A;
    type B = B;
    fn test_traverse<P>(self, processor: P)
    where
        P: PassedIn<Self::A>,
        P::Visited: PassedIn<Self::B>,
    {
    }
}

pub trait PassedIn<T> {
    type Visited;
}
