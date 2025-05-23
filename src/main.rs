mod cmd;
mod utils;
mod yaml;

extern crate diff;
use seahorse::App;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .command(cmd::scatter_cmd())
        .command(cmd::gather_cmd())
        .command(cmd::status_cmd())
        .command(cmd::clean_cmd())
        .command(cmd::echo_cmd());

    app.run(args);

    Ok(())
}
