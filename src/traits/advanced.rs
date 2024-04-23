
pub trait Processor {
    type Input;
    type Output;
    fn process(&self, input: Self::Input) -> Self::Output;
}


#[derive(Debug)]
pub struct Adder;

impl Processor for Adder {
    type Input = (i32, i32);
    type Output = i32;

     fn process(&self, input: Self::Input) -> Self::Output {
        input.0 + input.1
    }
}

pub fn process_and_print<P>(processor: &P, input: P::Input)
where
    P: Processor,
    P::Input: std::fmt::Debug,
    P::Output: std::fmt::Display,
{
    println!("Processed input: {:?}", input);
    let output = processor.process(input);
    println!("Output: {}", output);
}
