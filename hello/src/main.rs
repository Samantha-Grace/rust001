// fn main(keyword: i32) {

//     if keyword % 3 == 0 {
//         print!("fizz");
//     } else if keyword % 5 == 0  {
//         print!("buzz");
//     } else if keyword % 3 == 0 && keyword % 5 ==0 {
//         print!("fizzbuzz");
//     } else {
//         print!("{}", keyword);
//     }
//     }

fn main() {
    let number = 100;

    if number % 3 == 0 {
        print!("fizz");
    } else if number % 5 == 0  {
        print!("buzz");
    } else if number % 3 == 0 && number % 5 ==0 {
        print!("fizzbuzz");
    } else {
        print!("{}", number);
    }
    }
