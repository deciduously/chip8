use anyhow::Result;
use chip8::Machine;

fn init_renderer() {
    // TODO
}

fn init_input() {
    // TODO
}

fn init() {
    init_renderer();
    init_input();
}

fn main() -> Result<()> {
    init();

    let mut machine = Machine::new();
    machine.load_game("pong")?;
    if let Err(e) = machine.run() {
        eprintln!("Error: {}", e);
    }
    Ok(())
}
