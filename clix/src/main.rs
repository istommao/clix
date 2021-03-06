extern crate clap;

use std::fs;
use std::process::Command;

use clap::{Arg, App, SubCommand};


fn main() -> std::io::Result<()> {
    let home = std::env::var("HOME").unwrap();
    // let base_dir = format!("{}/.clix/scripts", home)

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

        // show
        .subcommand(SubCommand::with_name("show")
            .about("show Command alias")
            .arg(Arg::with_name("showtype")
                .value_name("showtype")
                .required(false)
                .help("show list")))

        // rename
        .subcommand(SubCommand::with_name("rename")
            .about("Rename Command alias")
            .arg(Arg::with_name("oldname")
                .value_name("oldname")
                .required(true))
            .arg(Arg::with_name("newname")
                .value_name("newname")
                .required(true))
            .help("clix rename old_name new_name"))

        // new
        .subcommand(SubCommand::with_name("new")
            .about("New Command alias")
            .arg(Arg::with_name("name")
                .value_name("name")
                .required(true))
            .arg(Arg::with_name("command")
                .value_name("command")
                .required(true))
            .help("clix new alias_name command"))

        // update
        .subcommand(SubCommand::with_name("update")
            .about("Update Command alias")
            .arg(Arg::with_name("name")
                .value_name("name")
                .required(true))
            .arg(Arg::with_name("command")
                .value_name("command")
                .required(true))
            .help("clix update alias_name command"))

        // run
        .subcommand(SubCommand::with_name("run")
            .about("Run Command")
            .arg(Arg::with_name("name")
                .value_name("name")
                .required(true))
            .help("clix run name"))

        .get_matches();

    // Show subcommand
    if let Some(matches) = matches.subcommand_matches("show") {
        if matches.is_present("showtype") {
            let showtype = matches.value_of("showtype").unwrap_or("");
            let cmdfile = format!("{}/{}.sh", format!("{}/.clix/scripts", home), showtype);
    
            let output = Command::new("bat")
                .arg(cmdfile)
                .output()
                .expect("failed to execute process");
    
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{}", output_str);
        } else {
            println!("=== show all Command alias ===");
            let paths = fs::read_dir(format!("{}/.clix/scripts", home)).unwrap();

            for path in paths {
                println!("{}", path.unwrap().path().display())
            }
        }
    }

    // Rename subcommand
    if let Some(matches) = matches.subcommand_matches("rename") {
        let oldname = matches.value_of("oldname").unwrap_or("");
        let newname = matches.value_of("newname").unwrap_or("");

        println!("Rename alias oldname: {} => newname: {}", oldname, newname);
        let oldpath = format!("{}/{}.sh", format!("{}/.clix/scripts", home), oldname);
        let newpath = format!("{}/{}.sh", format!("{}/.clix/scripts", home), newname);

        fs::rename(oldpath, newpath)?;
    }

    // Create new alias subcommand
    if let Some(matches) = matches.subcommand_matches("new") {
        let aliasname = matches.value_of("name").unwrap_or("");
        let command = matches.value_of("command").unwrap_or("");

        println!("Create alias name: {} => Command: {}", aliasname, command);

        let filename: String = format!("{}/{}.sh", format!("{}/.clix/scripts", home), aliasname);
        fs::write(filename, command).expect("Unable to write command to file");
    }

    // Update alias subcommand
    if let Some(matches) = matches.subcommand_matches("update") {
        let aliasname = matches.value_of("name").unwrap_or("");
        let command = matches.value_of("command").unwrap_or("");

        println!("Update alias name: {} => Command: {}", aliasname, command);

        let filename: String = format!("{}/{}.sh", format!("{}/.clix/scripts", home), aliasname);
        fs::write(filename, command).expect("Unable to write command to file");
    }

    // Run command
    if let Some(matches) = matches.subcommand_matches("run") {
        let aliasname = matches.value_of("name").unwrap_or("");
        // println!("Run Command {}\n", aliasname);

        let cmdfile = format!("{}/{}.sh", format!("{}/.clix/scripts", home), aliasname);

        let output = Command::new("sh")
            .arg(cmdfile)
            .output()
            .expect("failed to execute process");

        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }

    Ok(())
}