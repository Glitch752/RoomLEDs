use clap::arg;
use config::Configuration;

mod music_visualizer;
mod temporary_effect;
mod config;

// TODO: Retrieve from server?
static TOTAL_PIXELS: u32 = 812;

// TODO: Config file instead of passing the address everywhere

fn main() {
    let cmd = clap::Command::new("roomlightsctl")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommands([
            clap::command!("music-visualizer"),
            clap::command!("temporary-effect")
                .arg(arg!(<EFFECT> "The effect to run")),
            config::command()
        ]);
    
    let matches = cmd.get_matches();
    let address = Configuration::load().controller_ip;

    match matches.subcommand() {
        Some(("music-visualizer", _matches)) => music_visualizer::run(
            address
        ),
        Some(("temporary-effect", matches)) => temporary_effect::run(
            address,
            matches.get_one::<String>("effect").expect("Effect is required")
        ),
        Some(("config", matches)) => config::run(matches),
        _ => unreachable!()
    };
}
