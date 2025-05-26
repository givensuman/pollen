mod cmd;
mod utils;
mod yaml;

use seahorse::App;

use std::env;
use std::error::Error;

//                      ,,    ,,
//                    `7MM  `7MM
//                      MM    MM
// `7MMpdMAo.  ,pW"Wq.  MM    MM  .gP"Ya `7MMpMMMb.
//   MM   `Wb 6W'   `Wb MM    MM ,M'   Yb  MM    MM
//   MM    M8 8M     M8 MM    MM 8M""""""  MM    MM
//   MM   ,AP YA.   ,A9 MM    MM YM.    ,  MM    MM
//   MMbmmd'   `Ybmd9'.JMML..JMML.`Mbmmd'.JMML  JMML.
//   MM
// .JMML.
//
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let app = App::new("pollen")
        .description("🐝 a friendly dotfiles manager")
        .author("github.com/givensuman")
        .usage("pollen [command] [args]")
        .command(cmd::scatter_cmd())
        .command(cmd::gather_cmd())
        .command(cmd::status_cmd())
        .command(cmd::clean_cmd())
        .command(cmd::echo_cmd());

    app.run(args);

    Ok(())
}
