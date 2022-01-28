


use std::ops::{Index, IndexMut};
use std::fmt::{self, Display};

type ArgsIndex = usize;
struct Args(Vec<Arg>);


struct Arg {
    desc: String,
    value: Box<dyn Display>,
    mandatory: bool,
}


impl IndexMut<usize> for Args {

    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }

}


impl Index<usize> for Args {

    type Output = Arg;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }

}


impl Iterator for Args {
    type Item = Arg;

    fn next(&mut self) -> Option<Self::Item> {
        self.into_iter().next()
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        self.0.len()
    }
}


impl Arg {

    fn new(desc: String, default: Box::<dyn Display>) -> Self {
        Arg {
            desc,
            value: default,
            mandatory: false,
        }
    }


}


impl Args { 

    fn new() -> Args {
        Args {
            0: Vec::new(),
        }
    }

    fn add(&mut self, desc: String, default: Box::<dyn Display>) -> ArgsIndex {
        let idx = self.len();
        self.0.push(Arg::new(desc, default));
        idx
    }

    fn new_str(&mut self, desc: &str, default: &str) -> ArgsIndex {
        self.add(desc.to_string(), Box::new(default.to_string()))
    }

    fn new_bool(&mut self, desc: &str, default: bool) -> ArgsIndex {
        self.add(desc.to_string(), Box::new(default))
    }

    fn new_uint(&mut self, desc: &str, default: usize) -> ArgsIndex {
        self.add(desc.to_string(), Box::new(default))
    }



}

fn main() {
    let mut args = Args::new();

    let bool_idx = args.new_bool("First Arg", true);
    let uint_idx = args.new_uint("Second Arg", 5);
    let str_idx = args.new_str("Third Arg", "Hello, world");

    for arg in args.0 {
        println!("{}", arg.value);

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_added_to_args() {

        let mut args = Args::new();

        let str_idx = args.new_str("This is the hello arg." , "Hello");
        let str_idx2 = args.new_str("This is the world arg.", "World");

        assert_eq!(format!("{}", args.0[0].value), format!("{}", args[str_idx].value));
        assert_eq!(format!("{}", args.0[1].value), format!("{}", args[str_idx2].value));
        
    }

    #[test]
    fn test_bools_added_to_args() {
        let mut args = Args::new();

        let bool_idx = args.new_bool("This is the false arg." , false);
        let bool_idx2 = args.new_bool("This is the true arg.", true);

        assert_eq!(format!("{}", args.0[0].value), format!("{}", args[bool_idx].value));
        assert_eq!(format!("{}", args.0[1].value), format!("{}", args[bool_idx2].value));
        

    }

    #[test]
    fn test_ints_added_to_args() {
        let mut args = Args::new();

        let uint_idx = args.new_uint("This is the false arg." , 6);
        let uint_idx2 = args.new_uint("This is the true arg.", 5);

        assert_eq!(format!("{}", args.0[0].value), format!("{}", args[uint_idx].value));
        assert_eq!(format!("{}", args.0[1].value), format!("{}", args[uint_idx2].value));
        

    }
}
