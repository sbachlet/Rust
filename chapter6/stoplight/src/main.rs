// fn driving_action(light_color: &Stoplight) -> () {
//     match light_color {
//         Stoplight::Green => println!("Time to go."),
//         Stoplight::Yellow => println!("Woah! Better slow down."),
//         Stoplight::Red => println!("Guess I'll stop."),
//     };
// }

// enum Stoplight {
//     Red,
//     Green,
//     Yellow,
// }

// fn main() {
//     let x = Stoplight::Green;
//     let y = Stoplight::Yellow;
//     let z = Stoplight::Red;

//     driving_action(&x);
//     driving_action(&y);
//     driving_action(&z);
    
// }

// enum Maybe<T> {
//     Nada,
//     Yep(T),
// }

// fn main() {
//     let x = Maybe::Yep(5);
//     let y: Maybe<i32> = Maybe::Nada;
// }


// fn main () {
//     let x = Some(5);
//     let y: Option<i32> = None;
// }
use std::io;

enum Either<A, B> {
    Left(A),
    Right(B),
}

fn parse_int(x: &mut String) -> Either<String, i32> {
    match x.trim().parse() {
        Ok(num) => Either::Right(num),
        Err(_) => {
            Either::Left(format!("{} can't be parsed", x.trim()))
        },
    }
}



fn main() {
    let mut x = String::new();

    println!("Please enter a number");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    let value0: Either<String, i32> = parse_int(&mut x);
    match value0 {
        Either::Left(a) => {
            println!("{} occured please try again later", a);
        }
        Either::Right(b) => {
            println!("{} + 5 = {} ", b, b+5 );
        }
    }
}