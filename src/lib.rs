mod flag_parser {
    use std::env;
    use std::error::Error;
    use std::fmt::Display;
    use std::iter::Iterator;
    use std::str::FromStr;

    type FlagsIndex = usize;
    pub struct Flags(Vec<Flag>);

    pub struct Flag {
        name: String,
        desc: String,
        value: Box<dyn Display>,
        mandatory: bool,
        flag_type: FlagType,
    }

    enum FlagType {
        STR,
        UINT,
        BOOL,
    }

    impl Flag {
        fn new(name: String, desc: String, default: Box<dyn Display>, flag_type: FlagType) -> Self {
            Flag {
                name,
                desc,
                value: default,
                mandatory: false,
                flag_type,
            }
        }
    }

    impl Flags {
        pub fn new() -> Flags {
            Flags { 0: Vec::new() }
        }

        fn add(
            &mut self,
            name: String,
            desc: String,
            default: Box<dyn Display>,
            flag_type: FlagType,
        ) -> FlagsIndex {
            let idx = self.0.len();
            self.0.push(Flag::new(name, desc, default, flag_type));
            idx
        }

        pub fn new_str(&mut self, name: &str, desc: &str, default: &str) -> FlagsIndex {
            self.add(
                name.to_string(),
                desc.to_string(),
                Box::new(default.to_string()),
                FlagType::STR,
            )
        }

        pub fn new_bool(&mut self, name: &str, desc: &str, default: bool) -> FlagsIndex {
            self.add(
                name.to_string(),
                desc.to_string(),
                Box::new(default),
                FlagType::BOOL,
            )
        }

        pub fn new_uint(&mut self, name: &str, desc: &str, default: usize) -> FlagsIndex {
            self.add(
                name.to_string(),
                desc.to_string(),
                Box::new(default),
                FlagType::UINT,
            )
        }

        pub fn value(&self, idx: FlagsIndex) -> &Box<dyn Display> {
            &self.0[idx].value
        }

        fn change_value(&mut self, idx: FlagsIndex, arg: String) {
                self.0[idx].value = Box::new(arg);
        }

        fn contains(&self, flag: &String) -> Option<FlagsIndex> {
            for (idx, c) in flag.chars().enumerate() {
                if c == '-' {
                    return Some(idx);
                }
            }
            None
        }

        pub fn parse<I>(&mut self, mut args: I)
        where
            I: Iterator<Item = String>,
        {
            let mut flag_idx: FlagsIndex = 0;
            args.next();

            for mut arg in args {
                if arg.contains("-") {
                    arg.remove(0);
                    if let Some(idx) = self.contains(&arg) {
                        flag_idx = idx;
                    }
                } else {
                    self.change_value(flag_idx, arg);
                }
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::flag_parser::*;

        #[test]
        fn flag_can_be_found_in_flag_list() {
            let mut flags = Flags::new();
            let bool_idx = flags.new_bool("boolean", "boolean flag", false);

            flags.contains(&"-boolean".to_string()).unwrap();

        }

        #[test]
        fn flag_value_can_be_changed() {
            let mut flags = Flags::new();
            let bool_idx = flags.new_bool("boolean", "boolean flag", false);

            flags.change_value(bool_idx, "true".to_string());
            assert_eq!("true", format!("{}", flags.value(bool_idx)));
        }

        #[test]
        fn args_can_be_parsed_into_flags() {
            let args = vec!["cli-arg-parser", "-boolean", "true", "-uint", "5"];
            let args: Vec<String> = args.iter().map(|elem| elem.to_string()).collect();

            let mut flags = Flags::new();
            let bool_idx = flags.new_bool("boolean", "boolean flag", false);
            let uint_flag = flags.new_uint("uint", "uint flag", 6);

            flags.parse(args.into_iter());

            assert_eq!("true", format!("{}", flags.value(bool_idx)));
            assert_eq!("5", format!("{}", flags.value(uint_flag)));
        }

        #[test]
        fn strings_added_to_flags() {
            let mut flags = Flags::new();

            let str_idx = flags.new_str("H", "This is the hello Flag.", "Hello");
            let str_idx2 = flags.new_str("W", "This is the world Flag.", "World");

            assert_eq!("Hello", format!("{}", flags.value(str_idx)));
            assert_eq!("World", format!("{}", flags.value(str_idx2)));
        }

        #[test]
        fn bools_added_to_flags() {
            let mut flags = Flags::new();

            let bool_idx = flags.new_bool("F", "This is the false Flag.", false);
            let bool_idx2 = flags.new_bool("T", "This is the true Flag.", true);

            assert_eq!("false", format!("{}", flags.value(bool_idx)));
            assert_eq!("true", format!("{}", flags.value(bool_idx2)));
        }

        #[test]
        fn uints_added_to_flags() {
            let mut flags = Flags::new();

            let uint_idx = flags.new_uint("6", "This is the false Flag.", 6);
            let uint_idx2 = flags.new_uint("5", "This is the true Flag.", 5);

            assert_eq!("6", format!("{}", flags.value(uint_idx)));
            assert_eq!("5", format!("{}", flags.value(uint_idx2)));
        }
    }
}
