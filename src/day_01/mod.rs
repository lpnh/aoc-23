use std::error::Error;

use crate::Advent;

mod part_1;
mod part_2;

pub fn x_mas() -> Result<(), Box<dyn Error>> {
    let mut advent = Advent::ho_ho_ho()?;

    let _ = part_1::update(&mut advent);
    let _ = part_2::update(&mut advent);

    Ok(())
}
