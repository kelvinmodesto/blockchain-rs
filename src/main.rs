mod blockchain;
mod cli;
use blockchain::blockchain::Blockchain;
use cli::Cli;

pub type Result<T> = std::result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let cli = Cli::new();
    let _ = cli?.run();

    Ok(())
}
