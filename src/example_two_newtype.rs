pub trait TestTrait<P>
where
    P: PassedIn<Self::A>,
    P::Visited: PassedIn<Self::B>,
{
    type A;
    type B;

    fn test_traverse(self, processor: P);
}
struct Ab<A, B>((A, B));

impl<A, B, P> TestTrait<P> for Ab<A, B>
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
