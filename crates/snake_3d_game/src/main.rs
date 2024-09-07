use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    futures::executor::block_on(snake_3d_engine::run())?;
    Ok(())
}
