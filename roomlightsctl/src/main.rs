use clap::arg;

mod music_visualizer;

// TODO: Retrieve from server?
static TOTAL_PIXELS: u32 = 812;

fn main() {
    let cmd = clap::Command::new("roomlightsctl")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            clap::command!("music-visualizer")
                .arg(arg!(-a --address <ADDRESS> "The address of the server to connect to"))
                .arg_required_else_help(true)
        );
    
    let matches = cmd.get_matches();
    
    match matches.subcommand() {
        Some(("music-visualizer", matches)) => music_visualizer::run(
            matches.get_one::<String>("address").unwrap()
        ),
        _ => unreachable!()
    };
}
