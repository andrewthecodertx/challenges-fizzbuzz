fn main() {
    let mut number_now = 0u32;
// Starting at 0 sounds sane to me.
    loop {
        number_now += 1;
// I need to place the edge case check first.
        if number_now == 101 {
            break;
        }
        
        if !check_both(number_now) {
            if number_now % 3 == 0 {
                println!("Fizz");
            } else if number_now % 5 == 0 {
                println!("Buzz");
            } else {
                println!("{}", number_now);
            }
        }
    }
}

/* I feel the function below is an over-kill but
 at the same time it looks like the cleanest 
implementation I can think of.
My first attempt was not clean at all */
fn check_both(number_now: u32) -> bool {
    if number_now % 3 == 0 && number_now % 5 == 0 {
        println!("FizzBuzz");
        return true;
    }
    false
}
// It works!!!


// That was a lot. I'm also thinking of the
// for loop with pattern matching

/*
fn main() {
    for number in 1..=100 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", number),
        }
    }
}

*/
