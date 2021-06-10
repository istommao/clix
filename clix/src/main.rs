extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("clix")
        .version("0.1")
        .author("codingcat <istommao@gmail.com>")
        .about("Command alias extension\nGitHub: https://github.com/istommao/clix")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))

        .subcommand(SubCommand::with_name("show")
            .about("show Command alias list")
            .arg(Arg::with_name("showtype")
                .value_name("showtype")
                .required(true)
                .help("show list")))

        .subcommand(SubCommand::with_name("rename")
            .about("Rename Command alias")
            .arg(Arg::with_name("oldname")
                .value_name("oldname")
                .required(true))
            .arg(Arg::with_name("newname")
                .value_name("newname")
                .required(true))
            .help("clix rename old_name new_name"))

        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    if let Some(matches) = matches.subcommand_matches("show") {
        if matches.is_present("showtype") {
            let showtype = matches.value_of("showtype").unwrap_or("");
            println!("show Command alias name {}", showtype);
        } else {
            println!("show Command alias");
        }
    }

    if let Some(matches) = matches.subcommand_matches("rename") {
        let oldname = matches.value_of("oldname").unwrap_or("");
        let newname = matches.value_of("newname").unwrap_or("");

        println!("Rename alias oldname: {} => newname: {}", oldname, newname);
    }

}