mod clean;
mod echo;
mod gather;
mod scatter;
mod status;

use clean::clean;
use echo::echo;
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

pub fn clean_cmd() -> Command {
    Command::new("clean")
        .description("Clean command")
        .action(clean)
}

pub fn echo_cmd() -> Command {
    Command::new("echo")
        .description("Echo command")
        .action(echo)
}
