use clap::arg;

mod music_visualizer;
mod temporary_effect;

// TODO: Retrieve from server?
static TOTAL_PIXELS: u32 = 812;

// TODO: Config file instead of passing the address everywhere

fn main() {
    let cmd = clap::Command::new("roomlightsctl")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommands([
            clap::command!("music-visualizer")
                .arg(arg!(-a --address <ADDRESS> "The address of the server to connect to"))
                .arg_required_else_help(true),
            clap::command!("temporary-effect")
                .arg(arg!(-e --effect <EFFECT> "The effect to run"))
                .arg(arg!(-a --address <ADDRESS> "The address of the server to connect to"))
                .arg_required_else_help(true)
        ]);
    
    let matches = cmd.get_matches();
    
    match matches.subcommand() {
        Some(("music-visualizer", matches)) => music_visualizer::run(
            matches.get_one::<String>("address").unwrap()
        ),
        Some(("temporary-effect", matches)) => temporary_effect::run(
            matches.get_one::<String>("address").expect("Address is required"),
            matches.get_one::<String>("effect").expect("Effect is required")
        ),
        _ => unreachable!()
    };
}
