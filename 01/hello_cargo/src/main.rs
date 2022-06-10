use std::fmt;

fn display() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{number:0>width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 6;

    println!("{number:>width$}", number = number, width = width);
}

fn debug() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?} months in a year.", 12);

    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Deep(DebugPrintable(3)));
}

struct Structure(i32);

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let vec = &self.0;

        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    // display();
    // debug();

    let s = Structure(3231232);
    println!("{}", s);

    let v = List(vec![1, 2, 3, 4, 5]);
    println!("{}", v);
}
