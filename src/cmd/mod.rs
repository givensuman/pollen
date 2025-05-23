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
        .description("update system with pollen directory dotfiles")
        .usage("scatter [names]")
        .action(scatter)
}

pub fn status_cmd() -> Command {
    Command::new("status")
        .description("show diff status of pollen directory against system")
        .usage("status [names]")
        .action(status)
}

pub fn gather_cmd() -> Command {
    Command::new("gather")
        .description("update pollen directory with system dotfiles")
        .usage("gather [names]")
        .action(gather)
}

pub fn clean_cmd() -> Command {
    Command::new("clean")
        .description("delete tracked dotfiles from the system")
        .usage("clean [names]")
        .action(clean)
}

pub fn echo_cmd() -> Command {
    Command::new("echo")
        .description("echo to stdout the path of tracked dotfiles")
        .usage("echo [names]")
        .action(echo)
}
