use std::str::FromStr;

use crate::Prompt;

pub fn input<T: FromStr>(msg: &str) -> Result<T, T::Err> {
    Prompt::new().msg(msg).get()
}

pub fn input_string(msg: &str) -> String {
    Prompt::new().msg(msg).get_string()
}

impl Prompt<'_> {
    pub fn read<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.get()
    }

    pub fn read_str(&mut self) -> &str {
        self.get_str()
    }

    pub fn read_string(self) -> String {
        self.get_string()
    }

    pub fn read_until_ok<A, Ef>(&mut self, handle_error: Ef) -> A
    where
        A: FromStr,
        Ef: FnMut(A::Err),
    {
        self.get_until_ok(handle_error)
    }

    pub fn read_until_ok_with<T, E, F, Ef>(&mut self, action: F, handle_error: Ef) -> T
    where
        F: FnMut(&str) -> Result<T, E>,
        Ef: FnMut(E),
    {
        self.get_until_ok_with(action, handle_error)
    }

    pub fn read_line<T: FromStr>(&mut self) -> Result<T, T::Err> {
        self.get()
    }
}
