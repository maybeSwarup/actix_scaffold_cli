mod scaffolder;
mod template;

use clap::{ Arg, ArgAction, Command };
use scaffolder::{ generate_middleware, generate_mod, generate_route, generate_utils };

fn main() {
    let matches = Command::new("Actix Scaffold CLI")
        .about("Generate Actix-Web scaffolding for routes, modules, middleware, and utils.")
        .subcommand_required(true)
        .subcommand(
            Command::new("generate")
                .about("Generate scaffolding")
                .arg(Arg::new("type").required(true))
                .arg(Arg::new("name").required(true))
                .arg(Arg::new("no-inject").long("no-inject").action(ArgAction::SetTrue))
                .arg(Arg::new("with-test").long("with-test").action(ArgAction::SetTrue))
        )
        .get_matches();

    if let Some(sub_m) = matches.subcommand_matches("generate") {
        let scaffold_type = sub_m.get_one::<String>("type").unwrap().as_str();
        let name = sub_m.get_one::<String>("name").unwrap();
        let no_inject = sub_m.get_flag("no-inject");
        let with_test = sub_m.get_flag("with-test");

        match scaffold_type {
            "mod" => generate_mod(name, no_inject, with_test),
            "route" => generate_route(name, no_inject, with_test),
            "middleware" => generate_middleware(name, no_inject, with_test),
            "utils" => generate_utils(name, no_inject, with_test),
            _ => eprintln!("Unknown generate type."),
        }
    }
}
