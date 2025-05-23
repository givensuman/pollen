mod cmd;
mod utils;
mod yaml;

extern crate diff;
use seahorse::App;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new("pollen")
        .description("🐝 a friendly dotfiles manager")
        .author("github.com/givensuman")
        .usage("cli [args]")
        .command(cmd::scatter_cmd())
        .command(cmd::gather_cmd())
        .command(cmd::status_cmd())
        .command(cmd::clean_cmd())
        .command(cmd::echo_cmd());

    app.run(args);

    Ok(())
}
