// ============================================================
// Flow of Control in Rust
// Based on: https://doc.rust-lang.org/rust-by-example/flow_control.html
// ============================================================
//
pub fn if_else() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    // if/else can be used as an expression
    let big_n = if n < 10 && n > -10 {
        println!("and is a small number, increase ten-fold");
        10 * n
    } else {
        println!("and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

pub fn loop_example() {
    let mut count = 0u32;

    // Infinite loop with break
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue; // skip the rest of this iteration
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    // Loops can return a value via break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);
}

pub fn while_loop() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

pub fn for_range() {
    // for and range - 1..101 is exclusive of 101
    for n in 1..=5 {
        if n % 2 == 0 {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }

    // Iterating over a collection
    let names = vec!["Alice", "Bob", "Carol"];

    // iter() borrows each element
    for name in names.iter() {
        println!("Hello, {}!", name);
    }

    // into_iter() consumes the collection
    let cities = vec!["London", "Tokyo", "Sydney"];
    for city in cities.into_iter() {
        println!("City: {}", city);
    }
}

pub fn match_example() {
    let number = 13;

    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // match is an expression too
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
