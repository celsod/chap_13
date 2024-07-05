use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) //the (||... is the closure part of the code
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user preference {:?} gets {:?}", user_pref2, giveaway2);

    let mut list = vec![1, 2, 3];
    println!("List before closure: {:?}", list);

    let mut borrows_mutably = || list.push(7); 

    borrows_mutably();
    println!("After closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    /* The closure .map creates a new iterator that takes each item from the vector and 
    will increment each item by 1.  The collect method consumes the iterator and collects the
    resulting values into a collection data type. Can't call v1 below because v1 is consumed
    by the .map closure */

    assert_eq!(v2, vec![2, 3, 4]);

}