fn main() {
    let x = 5;      // x: i32                   // defined in main func block
    {
        let y = x;                              // nested block
        println!("{}", y);                      // block ends, y is dropped. no garbage collector. values are dropped when out of scope
    }
    println!("{}", x);                          // last print won't work
}



// shadow

fn main2() {
    let x = 5;      // x: i32                   // 
    {
        let x = 9;                              // x is shadowed with a new value
        println!("{}", x);                      // two x's with diff values   this prints 9, it is in it's own block
    }
    println!("{}", x);                          // prints 5
}

// shadow variables in the same scope

fn main3() {
    let mut x = 5;                              // x is mutable
    let x =x;                                   // x is now immutable
}