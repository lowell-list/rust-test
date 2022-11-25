fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}, and x is {x}");

    another_function(4);
    println!("The value of plus_one(five()) is: {}", plus_one(five()));

    if five() < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn another_function(x: i32) {
    println!("The value of x is: {x}"); // let's do this thing!
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
