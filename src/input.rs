use std::{
    io::{self, Stdin, Write},
    str::FromStr,
};

use crate::Ignore;

pub use ux::{input, input_string};

pub mod ux;

#[derive(Debug)]
pub struct Prompt<'msg> {
    message: Option<&'msg str>,
    buf: String,
    std_in: Stdin,
}

impl<'msg> Prompt<'msg> {
    pub fn msg(mut self, msg: &'msg str) -> Self {
        self.message = Some(msg);
        self
    }
}

impl Default for Prompt<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Prompt<'_> {
    pub fn new() -> Self {
        Self {
            message: None,
            buf: String::new(),
            std_in: io::stdin(),
        }
    }

    fn run(&mut self) {
        self.buf.clear();
        if let Some(message) = self.message {
            print!("{}", message);
            io::stdout().flush().ignore();
        }

        self.std_in.read_line(&mut self.buf).ignore();
    }

    pub fn get<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.get_str().parse()
    }

    pub fn get_str(&mut self) -> &str {
        self.run();
        self.buf.trim()
    }

    pub fn get_string(mut self) -> String {
        self.run();
        self.buf
    }

    pub fn get_until_ok<A, Ef>(&mut self, mut handle_error: Ef) -> A
    where
        A: FromStr,
        Ef: FnMut(A::Err),
    {
        self.get().unwrap_or_else(|err| {
            handle_error(err);
            self.get_until_ok(handle_error)
        })
    }

    pub fn get_until_ok_with<T, E, F, Ef>(&mut self, mut action: F, mut handle_error: Ef) -> T
    where
        F: FnMut(&str) -> Result<T, E>,
        Ef: FnMut(E),
    {
        action(self.get_str()).unwrap_or_else(|err| {
            handle_error(err);
            self.get_until_ok_with(action, handle_error)
        })
    }
}
