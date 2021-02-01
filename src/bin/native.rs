use anyhow::Result;
use chip8::{Machine, SdlContext};
use structopt::*;

#[derive(Debug, StructOpt)]
struct Opt {
    // /// Activate debug mode
    // #[structopt(short, long)]
    // debug: bool
    /// The name of the rom to load, lower-case
    #[structopt(short, long, default_value = "test_opcode")]
    rom_name: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Init context
    let context = SdlContext::new(15);
    let mut machine = Machine::new(context);
    machine.load_game(&opt.rom_name)?;
    if let Err(e) = machine.run() {
        eprintln!("Error: {}", e);
    }
    Ok(())
}
