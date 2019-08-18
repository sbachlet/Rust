#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32, 
}

pub fn add_two_length(rect: &mut Rectangle) -> () { 
    rect.length += 2;
}

pub fn rectangle_factory(length: u32, width: u32) -> Rectangle {
    if length == width {
        panic!("No squares allowed!");
    } else if length == width*2 {
        panic!("Invalid shape configuration.")
    }
    
    Rectangle{length, width}
}

#[cfg(test)]
mod good_tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rect = rectangle_factory(8, 7);
        add_two_length(&mut rect);
        assert_eq!(rect, Rectangle { length: 10, width: 7 });
    }

    #[test]
    fn it_works_2() {
        let mut rect = rectangle_factory(8, 7);
        add_two_length(&mut rect);
        assert_ne!(rect, Rectangle { length: 8, width: 7 });
    } 
}

#[cfg(test)]
mod bad_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "No squares allowed!")]
    fn square() {
        let x = rectangle_factory(5, 5);
    }

    #[test]
    #[should_panic(expected = "Invalid shape")]
    fn double_length() {
        let x = rectangle_factory(10, 5);
    }
}

// impl Rectangle {
//     pub fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.width > other.width
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle { length: 8, width: 7 }; 
//         let smaller = Rectangle { length: 5, width: 1 };
//         assert!(larger.can_hold(&smaller)); 
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle { length: 8, width: 7 };
//         let smaller = Rectangle { length: 5, width: 1 };
//         assert!(!smaller.can_hold(&larger));
//     }
// }
