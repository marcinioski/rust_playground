mod server;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    server::run()?;

    Ok(())
}
