fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    let fx = 2.0;
    let fy: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'z';
    let fancy = '∑';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (tx, ty, tz) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr = [1,2,3,4,5];
    let first = arr[0];
    let second = arr[1];
    //  Array out of bounds won't even compile.
    // This is a quality of life improvement that I've been saying should be added to many laguages for years.
    // let element = arr[10];

    println!("The value of ty is {}", ty);

    another_function(5, 6);

    let exp_y = {
        let x = 3;
        x + 1
    };

    let five_value = five();
    println!("The value of five_vale is {}", five_value);

    let six = plus_one(5);
    println!("The value of six is {}", six);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}