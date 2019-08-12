use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut buffer = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let set: HashSet<&char> = HashSet::from_iter(vowels.iter());
    
    println!("Please type a word.");
    io::stdin().read_line(&mut buffer).expect("Invalid input");
    buffer = buffer.trim().to_lowercase();

    match buffer.chars().next() {
        Some(x) => {
            if !set.contains(&x) {
                let x = &buffer[1..];
                let y = &buffer[0..1];
                println!("{}-{}ay", &x, &y[..]);
            } else {
                println!("{}-hay", buffer);
            };
        } 
        None => panic!(),
    }
}
