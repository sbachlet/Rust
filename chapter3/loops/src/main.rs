fn main() {
    let mut number = 3;
    let a = [10,20,30,40,50];
    let mut index = 0;

    while number != 0 {
        println!("{}", number);

        number = number - 1;
    }

    loop {
        println!("LIFTOFF!!!");
        break;    
    }

    // Compiler can't catch out of bounds errors in loops
    while index < 5 {
        println!("{}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("{}", element);
    }

    for number in (1..4).rev(){
        println!("{}", number)
    }
    println!("LIFTOFF!!!");
}
