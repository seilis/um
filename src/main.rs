mod os;
mod cli;

fn main() {
    let matches = cli::cli();
    println!("Hello, world from {:?}!", os::OperatingSystem::detect());
}
