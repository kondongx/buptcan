use clap::App;
use shadow_rs::shadow;

shadow!(shadow);

mod buptcan;

fn main() {
    let long_version = crate::shadow::clap_version();
    let matches = App::new("Bupt Campus Network Utils")
        .version(shadow::PKG_VERSION)
        .long_version(long_version.as_str())
        .author("Kevin K. <kondongx@gmail.com>")
        .about("Access Bupt Campus Network with your terminal")
        .subcommand(App::new("i").about("interact with commands"))
        .get_matches();

    // handle situation that cursor disappear when hit ctrlc
    ctrlc::set_handler(|| {
        std::process::exit(1);
    })
    .expect("Set ctrlc handle error");

    if let Some(("i", _)) = matches.subcommand() {
        buptcan::select_command()
    };
}
