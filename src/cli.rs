use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("spotify-cli")
        .version("0.1.0")
        .about("Your favorite music service in your terminal")
        .author("John-Olav Storvold")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("player")
                .about("player")
                .arg(
                    Arg::with_name("devices")
                        .short("d")
                        .long("devices")
                        .required(false)
                        .help("Retrieve devices")
                )
        )
}