mod gather;
mod scatter;
mod status;

use gather::gather;
use scatter::scatter;
use status::status;

use seahorse::Command;

pub fn scatter_cmd() -> Command {
    Command::new("scatter")
        .description("Scatter command")
        .action(scatter)
}

pub fn status_cmd() -> Command {
    Command::new("status")
        .description("Status command")
        .action(status)
}

pub fn gather_cmd() -> Command {
    Command::new("gather")
        .description("Gather command")
        .action(gather)
}
