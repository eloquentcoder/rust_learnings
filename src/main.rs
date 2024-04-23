pub mod traits {
    pub mod advanced;
    pub mod intermediate;
}

pub mod data_structures {
    pub mod index;
}

pub mod tests {
    pub mod adder;
}

pub mod closures {
    pub mod index;
}

pub mod pointers {
    pub mod smart;
}

use pointers::smart::main_function;

fn main() {
   main_function();
}
