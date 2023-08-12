use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};
use rand::Rng;

fn main() -> io::Result<()> {
    print!("Value to roll: ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    let mut rng = rand::thread_rng();
    io::stdin().read_line(&mut buffer)
        .expect("Error getting input");

    let v: Vec<&str> = buffer
        .trim()
        .split(|c| c == 'd' || c == 'D')
        .collect();

    if v.len() != 2 {
        let error = Error::new(ErrorKind::Other, "Invalid input");
        return Err(error);
    }

    let qty: u32 = v[0].parse().expect("Could not parse input");
    let sides: u32 = v[1].parse().expect("Could not parse input");

    let mut rolls: Vec<u32> = vec![];
    let mut total: u32 = 0;

    for _ in 1..=qty {
        let roll = &mut rng.gen_range(1..=sides);
        rolls.push(*roll);
        total += *roll;
    }

    print_result(rolls, total);

    Ok(())
}

fn print_result(rolls: Vec<u32>, total: u32) {
    println!("=====================");
    println!("You rolled:");
    for (idx, n) in rolls.iter().enumerate() {
        if (idx + 1) == rolls.len() {
            print!("{}", n);
        } else {
            print!("{}, ", n);
        }
    }
    io::stdout().flush().unwrap();
    println!();
    println!();
    println!("For a total of:");
    println!("{}", total);
}
