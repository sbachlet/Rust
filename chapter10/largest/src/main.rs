trait Display {
    fn to_str(&self) -> String {
        format!("Displaying self")
    }
}

struct Point3D {
    x: i32, 
    y: i32,
    z: i32,
}

impl Display for Point3D {}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn to_str(&self) -> String {
        format!("Point( x: {}, y: {})", self.x, self.y )
    }
}

fn print_two_big<T, U>(t: T, u: U) -> () 
    where T: Display,
          U: Display
{
    let text = t.to_str().to_uppercase();
    let text2 = u.to_str().to_uppercase();

    println!("{} {}", text, text2);
}

fn main() {
    let p = Point{x: 5, y: 5};
    let z = p.to_str();

    let p3 = Point3D{x: 5, y: 5, z: 1};
    let z3 = p3.to_str();

    print_two_big(p, p3)
}
// use std::fmt::Display;
// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
//     where T: Display {
//         println!("Announcement! {}", ann);
//         if x.len() > y.len() {
//             x
//         } else {
//             y 
//         }
//     }

// fn main() {
//     let x = "abc";
//     let y = "wxyz";
//     let announcement = 42;

//     let longest = longest_with_an_announcement(x, y, announcement);
//     println!("{}", longest)

// }
// #[derive(Debug)]
// struct ImportantExcerpt<'a> { 
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago..."); 
//     let i;
//      {
//         let first_sentence = novel.split('.')
//             .next()
//             .expect("Could not find a '.'");
//          i = ImportantExcerpt { part: first_sentence }
//      }
//     println!("{:?}", i);
// }

// fn main() {
//     let string1 = String::from("long string is long");
//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     } 
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }

//     println!("The longest string is {}", result);
// }

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else { 
        y
    } 
}
// fn main(){
//     let x = 5; 
//     let r = &x;
//     println!("r: {}", r);
// }
// fn main(){
//     let r;
//     {
//         let x = 5; 
//         r = &x;
//     }
//     println!("r: {}", r);
// }
// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let &mut largest = &mut list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     &largest
// }
// enum Type {
//     ThisWorks,
// }

// struct OtherType<A> {
//     ThisAlsoWorks: A,
// }


// fn vector_of_five<T>(t: &T) -> std::vec::Vec<&T> {
//     vec![t, t, t, t, t]
// }

// struct Point<T> {
//     x: T,
//     y: T, 
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x 
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };
//     println!("p.x = {}", p.x());

//     let v = vector_of_five(&5);
//     println!("{:?}", v);
// }

// fn main() {
//     let a: Result<String, i32> = Ok("This works".to_string());
//     let b: Result<i32, String> = Err("This also works".to_string());
//     let c: Result<i32, u32> = Ok(42);
//     let d: Result<i32, u32> = Err(42);
//     let e: Result<Type, OtherType<i32>> = Ok(Type::ThisWorks);
//     let f: Result<Type, OtherType<String>> = Err(OtherType {ThisAlsoWorks: "Try it".to_string()}); 
// }
