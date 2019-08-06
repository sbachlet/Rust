fn main() {
    let number = 5;

    println!("Fib 5 recursive is {}", fib_recursive(number));
    println!("Fib 5 looping is {}", fib_looping(&number));
    println!("Fib 5 looping is {}", fib_till(&number));
    println!("Fib 5 looping is {}", fib_while(&number));
}

// Don't bother passing reference because a new value is created each time
fn fib_recursive(x: i32) -> i32 {
    if x <= 1 {
        x
    } else {
        fib_recursive(x - 1) + fib_recursive(x - 2)
    }
}

// Pass references to avoid putting new values on the stack
fn fib_looping(x: &i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;

    for _ in 0..x-1 {
        c = a + b;
        a = b;
        b = c;   
    };
    c
}

fn fib_till(x: &i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut n = 0;

    loop {
        if n < x - 1 {
            c = a + b;
            a = b;
            b = c; 
            n = n + 1;  
        } else {
            break;
        }
    }

    c
}

fn fib_while(x: &i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut n = 0;

    while n < x - 1 {
        c = a + b;
        a = b;
        b = c;
        n = n + 1;   
    }

    c
}