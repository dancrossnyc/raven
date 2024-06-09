use std::io::Read;
use std::path::PathBuf;

use crow::Varvara;
use raven::{Uxn, UxnRam};

use anyhow::{Context, Result};
use clap::Parser;

/// Uxn runner
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    rom: PathBuf,
}

fn main() -> Result<()> {
    let env = env_logger::Env::default()
        .filter_or("UXN_LOG", "info")
        .write_style_or("UXN_LOG", "always");
    env_logger::init_from_env(env);

    let args = Args::parse();
    let mut f = std::fs::File::open(&args.rom)
        .with_context(|| format!("failed to open {:?}", args.rom))?;

    let mut rom = vec![];
    f.read_to_end(&mut rom).context("failed to read file")?;

    let mut ram = UxnRam::new();
    let mut vm = Uxn::new(&rom, &mut ram);
    let mut dev = Varvara::new();
    let start = std::time::Instant::now();
    vm.run(&mut dev, 0x100);
    println!("{:?}", start.elapsed());
    dev.run(&mut vm);

    Ok(())
}