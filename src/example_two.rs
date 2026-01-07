pub trait TestTrait<P>
where
    P: PassedIn<Self::A>,
    P::Visited: PassedIn<Self::B>,
{
    type A;
    type B;

    fn test_traverse(self, processor: P);
}

impl<A, B, P> TestTrait<P> for (A, B)
where
    P: PassedIn<Self::A>,
    P::Visited: PassedIn<Self::B>,
{
    type A = A;
    type B = B;
    fn test_traverse(self, processor: P) {}
}

pub trait PassedIn<T> {
    type Visited;
}
