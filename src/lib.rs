use std::{
    fmt,
    io::{self, Stdin, Write},
    str::FromStr,
};

pub fn input<T: FromStr>(msg: &str) -> Result<T, T::Err> {
    Input::new().msg(msg).get()
}

#[derive(Debug)]
pub struct Input<'msg> {
    message: Option<&'msg str>,
    buf: String,
    std_in: Stdin,
}

impl<'msg> Input<'msg> {
    pub fn msg(mut self, msg: &'msg str) -> Self {
        self.message = Some(msg);
        self
    }
}

impl Default for Input<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Input<'_> {
    pub fn new() -> Self {
        Self {
            message: None,
            buf: String::new(),
            std_in: io::stdin(),
        }
    }

    pub fn get<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.buf.clear();
        if let Some(message) = self.message {
            print!("{}", message);
            io::stdout().flush().ignore();
        }

        self.std_in.read_line(&mut self.buf).ignore();
        self.buf.trim().parse()
    }
}

// aliases
impl Input<'_> {
    pub fn read<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.get()
    }

    pub fn read_line<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.get()
    }
}

pub trait Ignorable {
    fn ignore(self);
}

impl<T, E> Ignorable for Result<T, E>
where
    T: fmt::Debug,
    E: fmt::Display,
{
    fn ignore(self) {
        match self {
            Ok(x) => log::trace!("ignoring {:?}", x),
            Err(e) => log::error!("ignoring error {}", e),
        }
    }
}

impl<T> Ignorable for Option<T>
where
    T: fmt::Debug,
{
    fn ignore(self) {
        if let Some(x) = self {
            log::trace!("ignoring {:?}", x)
        } else {
            log::error!("ignoring a None Value")
        }
    }
}
