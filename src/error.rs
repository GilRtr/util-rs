use std::{
    convert::Infallible,
    fmt::{Debug, Display},
};

pub trait Ignore {
    fn ignore(self);
}

impl<T, E> Ignore for Result<T, E>
where
    T: Debug,
    E: Display,
{
    fn ignore(self) {
        match self {
            Ok(x) => log::trace!("ignoring {:?}", x),
            Err(e) => log::error!("ignoring error {}", e),
        }
    }
}

impl<T> Ignore for Option<T>
where
    T: Debug,
{
    fn ignore(self) {
        if let Some(x) = self {
            log::trace!("ignoring {:?}", x)
        } else {
            log::error!("ignoring a None Value")
        }
    }
}

pub trait SafeUnwrap {
    type Inner;

    fn safe_unwrap(self) -> Self::Inner;
}

impl<T> SafeUnwrap for Result<T, Infallible> {
    type Inner = T;

    fn safe_unwrap(self) -> Self::Inner {
        self.unwrap_or_else(|err| match err {})
    }
}
