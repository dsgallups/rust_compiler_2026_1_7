pub trait TestTrait<P>
where
    P: PassedIn<Self::A>,
    P::Visited: PassedIn<Self::B>,
{
    type A;
    type B;

    fn test_traverse(self, processor: P);
}

struct Foo;
struct Bar;
struct Baz;

impl<P> TestTrait<P> for Foo
where
    P: PassedIn<Bar>,
    P::Visited: PassedIn<Baz>,
{
    type A = Bar;
    type B = Baz;
    fn test_traverse(self, processor: P) {}
}

pub trait PassedIn<T> {
    type Visited;
}
