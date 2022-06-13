fn main() {
    let mut s = String::from("hello");

    let x = 5;

    makes_copy(x);
    println!("{}", x);

    // borrow s to the fn
    // similar to refernce in c++

    // otherwise s will be dropped after call
    borrow(&s);
    println!("{}", s);

    // mutable borrow
    borrow_and_mutate(&mut s);

    println!("{}", s);
}

fn borrow_and_mutate(some_string: &mut String) {
    some_string.push_str(", world");
}

// some_string refers to some value but does NOT own it
fn borrow(some_string: &String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
