
// RUST: Safety Concurrency, Speedinitialize a single variable
// no type annotations
fn main() {
    let bunnies = 2;
}

// type
fn main() {
    let bunnies: i32 =4;
}

// let statement allows us to initialize multiple variables at once
// let statement can destructure data on right hand side 
// and use it to initialize variables inside of a corresponding pattern on the left side
fn main() {
    let(bunnies, carrots) = (8, 50);
}

// variables are immutable by default
fn main() {
    let bunnies = 16;
}

// will work
fn main() {
    let mut bunnies = 32;
    bunnies =2;
}