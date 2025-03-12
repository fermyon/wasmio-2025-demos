use bindings::wasmio::demo::sorting;
use rand::Rng;
use std::io;
use std::time::Instant;

mod bindings;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num: usize = if args.len() > 1 {
        match args[1].parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("Please enter a valid number higher than 0");
                return;
            }
        }
    } else {
        loop {
            println!("How many random numbers do you want to create?");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().parse() {
                Ok(n) if n > 0 => break n,
                _ => println!("Please enter a valid number higher than 0"),
            }
        }
    };

    let numbers: Vec<i32> = (0..num)
        .map(|_| rand::rng().random_range(i32::MIN..i32::MAX))
        .collect();

    let start = Instant::now();
    let res = sorting::sort(numbers.as_slice());
    let duration = start.elapsed();
    let duration_mu = duration.as_micros();

    println!("Sorted: {} numbers", res.len());
    println!(
        "Numbers are ranging from {} to {}",
        res.first().unwrap(),
        res.last().unwrap()
    );

    match duration_mu {
        d if d > 60000000 => println!("Took: {}m", duration.as_secs_f32() / 60.0f32),
        d if d > 1000000 => println!("Took: {}sec", duration.as_secs_f32()),
        d if d > 1000 => println!("Took: {}ms", duration.as_millis()),
        _ => println!("Took: {}mu", duration.as_micros()),
    }
}
