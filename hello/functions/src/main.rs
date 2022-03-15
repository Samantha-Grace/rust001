// use snake case, lower case words seperated by underscores
fn main() {
    do_stuff();
}

// functions don't have to appear in the file before code that calls them
fn do_stuff() {
    println!("Hello, world!");
}

// function parameters are always defined with with name, colon, type

fn do_stuff(qty: f64);

// multiple parameters are seperated by a comma
fn do_stuff(qty: f64, oz: f64);

// specify the return type after the parameters by adding an arrrow
fn do_stuff(qty: f64, oz: f64) -> f64;

// body is inside block
fn do_stuff(qty: f64, oz: f64) -> f64 {

}

// you can return a value from a func with return keyword
fn do_stuff(qty: f64, oz: f64) -> f64 {
    return qty * oz;
    //shorthand for return - leave the semicolon off (this is called a tail exprssion)
    qty * oz
}

// this is the same as this
// { return true; }
// { true }