use clap::{arg, command, value_parser, ArgAction, Command, ArgMatches};

pub fn cli() -> ArgMatches {
    let matches = command!()
        .about("A tool for managing users and dotfiles across machines")
        .subcommand(
            Command::new("user")
                .about("Manage users and accounts")
                .subcommand(
                    Command::new("new")
                        .about("Make a new user")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Disable a user and remove all records")
                )
                .subcommand(
                    Command::new("disable")
                        .about("Make a user account inactive, but don't delete from database")
                )
        )
        .subcommand(
            Command::new("file")
                .about("Manage files associated with a user account")
                .subcommand(
                    Command::new("add")
                        .about("Add a new file to be synced")
                )
                .subcommand(
                    Command::new("rm")
                        .about("Remove a file from syncing")
                )
        )
        .subcommand(
            Command::new("status")
                .about("Print the status of this system")
        )
        .subcommand(
            Command::new("sync")
                .about("Sync this setup with the remote")
        )
        .subcommand(
            Command::new("machine")
                .about("Manage devices that should be associated with this config")
                .subcommand(
                    Command::new("enrole")
                        .about("Enrole this system with a remote")
                )
        )
        .get_matches();
    matches
}
