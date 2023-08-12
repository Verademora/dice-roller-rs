use std::io::{self, Write, Error, ErrorKind};
use rand::Rng;

fn main() -> io::Result<()> {
    // Get user input
    print!("Value to roll: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // Split input assuming input is correct
    let v: Vec<&str> = buffer
        .trim()
        .split(|c| c == 'd' || c == 'D')
        .collect();

    // Check that we got 2 items from the split
    if v.len() != 2 {
        let error = Error::new(ErrorKind::InvalidInput, "Invalid input");
        return Err(error);
    }

    // Parse and roll dice
    let qty: u32 = v[0].parse()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Could not parse quantity"))?;
    let sides: u32 = v[1].parse()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "Could not parse sides"))?;

    let mut rolls: Vec<u32> = Vec::new();
    let mut total: u32 = 0;

    for _ in 0..qty {
        let roll = rand::thread_rng().gen_range(1..=sides);
        rolls.push(roll);
        total += roll;
    }

    // Print the result
    print_result(rolls, total);

    Ok(())
}

// Takes the final vector containing all the rolls and the total
// and prints the results
fn print_result(rolls: Vec<u32>, total: u32) {
    println!("=====================");
    println!("You rolled:");
    // for (idx, n) in rolls.iter().enumerate() {
    //     if (idx + 1) == rolls.len() {
    //         print!("{}", n);
    //     } else {
    //         print!("{}, ", n);
    //     }
    // }
    // io::stdout().flush().unwrap();
    println!("{:?}", rolls);
    println!();
    println!();
    println!("For a total of:");
    println!("{}", total);
}
