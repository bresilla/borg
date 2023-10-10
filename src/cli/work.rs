use clap::{arg, Command};

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("work")
        .about(aly("w", subs) + "Various workspace subcommands\n")
        .aliases(&["w", "pkg", "pack"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("create")
            .about("Create a new package")
            .aliases(["c", "new"])
            .arg_required_else_help(true)
            .arg(
                arg!(<PACKAGE_NAME> "Name of the ROS package to create (e.g. 'demo_nodes_cpp')")
                .required(true)
            )
            .arg(arg!(--package_format <PACKAGE_FORMAT> "The package.xml format."))
            .arg(arg!(--description <DESCRIPTION> "The description given in the package.xml"))
            .arg(arg!(--license <LICENSE> "The license attached to this package"))
            .arg(arg!(--destination_directory <DESTINATION_DIRECTORY> "Directory where to create the package directory"))
            .arg(arg!(--build_type <BUILD_TYPE> "The build type to process the package with"))
            .arg(arg!(--dependencies <DEPENDENCIES>... "List of dependencies"))
            .arg(arg!(--maintainer_email <MAINTAINER_EMAIL> "Email address of the maintainer of this package"))
            .arg(arg!(--maintainer_name <MAINTAINER_NAME> "Name of the maintainer of this package"))
            .arg(arg!(--node_name <NODE_NAME> "Name of the empty executable"))
            .arg(arg!(--library_name <LIBRARY_NAME> "Name of the empty library"))
        )
        .subcommand(
            Command::new("list")
            .about("List all packages")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
            .arg(arg!(-a --all "Display all packages even hidden ones"))
            .arg(arg!(-c --count_packages "Only display the number of packages discovered"))
        )
        .subcommand(
            Command::new("info")
            .about("Print information about a package")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                arg!(<PACKAGE_NAME> "Name of the ROS package to get info (e.g. 'demo_nodes_cpp')")
                .required(true)
            )
            .arg(arg!(--xml "Output the XML of the package manifest"))
        )
        .subcommand(
            Command::new("build")
            .about("Build a package")
            .aliases(["b", "make", "m"])
            .arg_required_else_help(true)
        )
}