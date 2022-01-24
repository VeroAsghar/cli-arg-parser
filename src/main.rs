


use std::ops::{Index, IndexMut};
use std::fmt::{self, Display};

struct Args(Vec<(String, Box<dyn Display>)>);


impl IndexMut<usize> for Args {

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }

}


impl Index<usize> for Args {

    type Output = (String, Box<dyn Display>);

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }

}


impl Iterator for Args {
    type Item = (String, Box<dyn Display>);

    fn next(&mut self) -> Option<Self::Item> {
        self.into_iter().next()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.0.len()
    }
}



impl Args { 

    fn new() -> Args {
        Args {
            0: Vec::new(),
        }
    }

    fn add(&mut self, desc: String, default: Box::<dyn Display>) {
        self.0.push((desc, default));
    }

    /// Returns index of argument in argument list
    fn new_string(&mut self, desc: &str, default: &str) -> usize {
        let idx = self.len();
        self.add(desc.to_string(), Box::new(default.to_string()));
        idx
    }

    fn new_usize(&mut self, desc: &str, default: usize) -> usize {
        let idx = self.len();
        self.add(desc.to_string(), Box::new(default));
        idx
    }

    fn new_bool(&mut self, desc: &str, default: bool) -> usize {
        let idx = self.len();
        self.add(desc.to_string(), Box::new(default));
        idx
    }


}


fn main() {

    let mut args = Args::new();

    let string = args.new_string("This is the hello arg." , "Hello");
    let string2 = args.new_string("This is the world arg.", "World");
    
    println!("{}, {}", args[string].1, args[string2].1);
    args[string].1 = Box::new("meow".to_string());
    println!("{}, {}", args[string].1, args[string2].1);

}
