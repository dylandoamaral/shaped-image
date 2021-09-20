pub mod genetic;
pub mod utils;

use crate::utils::cli::Opts;
use clap::Clap;
use genetic::population::Population;

fn main() {
    let opts: Opts = Opts::parse();

    match Population::new(opts) {
        Ok(mut population) => population.process(),
        Err(error) => {
            println!("{}", error);
        }
    };
}
