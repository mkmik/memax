use std::error::Error;

extern crate memax;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Logical RAM: {}", memax::get()?);
    println!("Physical RAM: {}", memax::get_physical()?);

    Ok(())
}
