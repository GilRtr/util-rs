use std::ops::{Deref, DerefMut};

pub struct Defer<T, F: FnOnce(T)>(Option<T>, Option<F>);

pub fn defer<T, F: FnOnce(T)>(x: T, f: F) -> Defer<T, F> {
    Defer(Some(x), Some(f))
}

impl<T, F: FnOnce(T)> Deref for Defer<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .expect("struct API does not allow this to be None (defer - deref - x)")
    }
}

impl<T, F: FnOnce(T)> DerefMut for Defer<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .as_mut()
            .expect("struct API does not allow this to be None (defer - derefmut - x)")
    }
}

impl<T, F: FnOnce(T)> Drop for Defer<T, F> {
    fn drop(&mut self) {
        self.1
            .take()
            .expect("struct API does not allow this to be None (defer - fn)")(
            self.0
                .take()
                .expect("struct API does not allow this to be None (defer - x)"),
        )
    }
}
