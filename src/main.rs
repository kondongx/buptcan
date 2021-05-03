use clap::App;

mod buptcan;

fn main() {
    let matches = App::new("Bupt Campus Network Utils")
        .version("0.1.0")
        .author("Kevin K. <kondongx@gmail.com>")
        .about("Access Bupt Campus Network with your terminal")
        .subcommand(App::new("i").about("interact with commands"))
        .get_matches();

    match matches.subcommand() {
        Some(("i", _)) => buptcan::select_command(),
        _ => {}
    }
}
