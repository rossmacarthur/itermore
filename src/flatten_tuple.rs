//! Provides a way to flatten tuples of the form `((A, B), C, D...)` into
//! `(A, B, C, D...)`

pub fn flatten_tuple<T: FlattenTuple>(t: T) -> T::Output {
    t.flatten_tuple()
}

pub trait FlattenTuple {
    type Output;
    fn flatten_tuple(self) -> Self::Output;
}

impl<A, B, C> FlattenTuple for ((A, B), C) {
    type Output = (A, B, C);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c) = self;
        (a, b, c)
    }
}

impl<A, B, C, D> FlattenTuple for ((A, B), C, D) {
    type Output = (A, B, C, D);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d) = self;
        (a, b, c, d)
    }
}

impl<A, B, C, D, E> FlattenTuple for ((A, B), C, D, E) {
    type Output = (A, B, C, D, E);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e) = self;
        (a, b, c, d, e)
    }
}

impl<A, B, C, D, E, F> FlattenTuple for ((A, B), C, D, E, F) {
    type Output = (A, B, C, D, E, F);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f) = self;
        (a, b, c, d, e, f)
    }
}

impl<A, B, C, D, E, F, G> FlattenTuple for ((A, B), C, D, E, F, G) {
    type Output = (A, B, C, D, E, F, G);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g) = self;
        (a, b, c, d, e, f, g)
    }
}

impl<A, B, C, D, E, F, G, H> FlattenTuple for ((A, B), C, D, E, F, G, H) {
    type Output = (A, B, C, D, E, F, G, H);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g, h) = self;
        (a, b, c, d, e, f, g, h)
    }
}

impl<A, B, C, D, E, F, G, H, I> FlattenTuple for ((A, B), C, D, E, F, G, H, I) {
    type Output = (A, B, C, D, E, F, G, H, I);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g, h, i) = self;
        (a, b, c, d, e, f, g, h, i)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> FlattenTuple for ((A, B), C, D, E, F, G, H, I, J) {
    type Output = (A, B, C, D, E, F, G, H, I, J);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g, h, i, j) = self;
        (a, b, c, d, e, f, g, h, i, j)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> FlattenTuple for ((A, B), C, D, E, F, G, H, I, J, K) {
    type Output = (A, B, C, D, E, F, G, H, I, J, K);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g, h, i, j, k) = self;
        (a, b, c, d, e, f, g, h, i, j, k)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> FlattenTuple for ((A, B), C, D, E, F, G, H, I, J, K, L) {
    type Output = (A, B, C, D, E, F, G, H, I, J, K, L);

    fn flatten_tuple(self) -> Self::Output {
        let ((a, b), c, d, e, f, g, h, i, j, k, l) = self;
        (a, b, c, d, e, f, g, h, i, j, k, l)
    }
}
