use std::io;

fn main() {
    println!("Hello, welcome to Fib");

    println!("Which fibonacci number would you like?");

    let mut num = String::new();

    let num: u32 = loop {
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read input");

        match num.trim().parse() {
            Ok(val) => break val,
            Err(_) => {
                println!(
                    "Failed to parse input as number.. Please input a reasonable positive number."
                );
                continue;
            }
        };
    };

    println!(
        "The {} fibonacci number is: {}",
        num,
        compute_nth_fib_number(num).expect("Input too large")
    );
}

fn compute_nth_fib_number(num: u32) -> Result<u128, &'static str> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = a + b;

    if 0 == num {
        return Ok(0);
    }

    for _ in 2..=num {
        c = match a.checked_add(b) {
            Some(val) => val,
            None => return Err("Fibonacci number grew too large.."),
        };
        a = b;
        b = c;
    }

    return Ok(c);
}
