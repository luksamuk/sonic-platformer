use anyhow::{Result, bail};
use std::env;

fn main() -> Result<()> {
    // How to use:
    // mapgen small.png smallsize big.png bigsize
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        bail!("Not enough parameters");
    }

    let smaller_image = &args[1];
    let smaller_image_size = args[2].parse::<usize>()?;
    let bigger_image = &args[3];
    let bigger_image_size = args[4].parse::<usize>()?;

    println!("Working dir: {:?}", env::current_dir()?);

    println!(
        "Small: {} ({}x{})",
        smaller_image, smaller_image_size, smaller_image_size
    );
    println!(
        "Big: {} ({}x{})",
        bigger_image, bigger_image_size, bigger_image_size
    );

    Ok(())
}
