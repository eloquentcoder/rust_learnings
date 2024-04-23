// Define a trait named SoundMaker
pub trait SoundMaker {
    fn make_sound(&self);
}

// Implement the SoundMaker trait for a Dog
pub struct Dog;
impl SoundMaker for Dog {
    fn make_sound(&self) {
        println!("Dog says woof!");
    }
}

pub struct Lion;
// Implement the SoundMaker trait for a Cat
pub struct Cat;
impl SoundMaker for Cat {
    fn make_sound(&self) {
        println!("Cat says meow!");
    }
}

// A function that takes a trait object Box<dyn SoundMaker>
pub fn make_some_noise(animal: &dyn SoundMaker) {
    animal.make_sound();
}

pub fn main_function() {
    let dog = Dog;
    let cat = Cat;

    // Convert the Dog and Cat into trait objects
    let dog_boxed: Box<dyn SoundMaker> = Box::new(dog);
    let cat_boxed: Box<dyn SoundMaker> = Box::new(cat);

    // Call the make_some_noise function with different animals
    make_some_noise(&*dog_boxed); // Output: Dog says woof!
    make_some_noise(&*cat_boxed); // Output: Cat says meow!
}
