mod gather;
mod scatter;

use gather::gather;
use scatter::scatter;

use seahorse::Command;

pub fn scatter_cmd() -> Command {
    Command::new("scatter")
        .description("Scatter command")
        .action(scatter)
}

pub fn gather_cmd() -> Command {
    Command::new("gather")
        .description("Gather command")
        .action(gather)
}
