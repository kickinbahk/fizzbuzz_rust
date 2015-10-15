fn main() {
    let mut range = 1..100;
    for x in range {
        match x {
            x if x % 15 == 0 => println!("FizzBuzz"),
            x if x % 3  == 0 => println!("Fizz"),
            x if x % 5  == 0 => println!("Buzz"),
            x => println!("{}", x),
        }
    }

 // FizzBuzz done with tuples
 //   
 //   fn main() {
 //       let mut range = 1..100;
 //       for x in range {
 //           match (x % 3, x % 5) {
 //               (0, 0) => println!("FizzBuzz"),
 //               (0, _) => println!("Fizz"),
 //               (_, 0) => println!("Buzz"),
 //               _ => println!("{}", x),
 //           }
 //       }
 //   }
}
