fn main() {
    fizzbuzz(15);
}

fn fizzbuzz(number: i32) {

    if number % 3 == 0 {
        println!("fizz");
    } else if number % 5 == 0  {
        println!("buzz");
    } else if number % 3 == 0 && number % 5 ==0 {
        println!("fizzbuzz");
    } else {
        println!("{}", number);
    }
    }




// fn main() {
//     let number = 15;

//     if number % 3 == 0 {
//         println!("fizz");
//     } else if number % 5 == 0  {
//         println!("buzz");
//     } else if number % 3 == 0 && number % 5 == 0 {
//         println!("fizzbuzz");
//     } else {
//         println!("{}", number);
//     }
//     }
