use std::fmt::Debug;

pub trait Printable {
    fn print(&self);
}

pub struct Wrapper<T: Printable> {
    value: T,
}

impl<T: Printable> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn print_name(&self) {
        self.value.print()
    }
}

impl Printable for i32 {
     fn print(&self) {
        println!("{}", self)
    }
}

impl Printable for &str {
    fn print(&self) {
        println!("{}", self)
    }
}

pub fn print_value<T>(value: T)
where
    T: Debug,
{
    println!("The value is: {:?}", value);
}
