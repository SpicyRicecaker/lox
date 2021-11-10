fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = nenia::main() {
        eprintln!("{}", e)
    }
    Ok(())
}
