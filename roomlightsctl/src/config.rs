use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "roomlightsctl.ron";

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Configuration {
    /// The IP address of the controller
    pub controller_ip: std::net::IpAddr,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            controller_ip: std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100))
        }
    }
}

impl Configuration {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .expect("Failed to get config directory")
            .join(CONFIG_FILE_NAME);
    
        // If the config file doesn't exist, return the default configuration
        if !config_path.exists() {
            return Configuration::default();
        }

        let file = std::fs::File::open(config_path)
            .expect("Failed to open config file");

        ron::de::from_reader(file)
            .expect("Failed to deserialize config file")
    }

    pub fn save(&self) {
        let config_path = dirs::config_dir()
            .expect("Failed to get config directory")
            .join(CONFIG_FILE_NAME);

        let file = std::fs::File::create(config_path)
            .expect("Failed to create config file");

        ron::ser::to_writer_pretty(file, self, Default::default())
            .expect("Failed to serialize config file");
    }
}

pub(crate) fn command() -> clap::Command {
    clap::command!("config")
        .subcommand_required(false)
        .subcommands([
            clap::command!("show"),
            clap::command!("set")
                .arg(clap::arg!(--ip <IP> "The IP address of the controller").required(true))
        ])
}

pub(crate) fn run(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("show", _matches)) => {
            let config = Configuration::load();
            println!("Controller IP: {}", config.controller_ip);
        },
        Some(("set", matches)) => {
            let ip = matches.get_one::<String>("ip").expect("IP is required")
                .parse()
                .expect("Failed to parse IP address");

            let config = Configuration { controller_ip: ip };
            config.save();

            println!("Controller IP set to {}", ip);
        },
        None => {
            let config_path = dirs::config_dir()
                .expect("Failed to get config directory")
                .join(CONFIG_FILE_NAME);

            println!("Config file path: {}", config_path.display());
        }
        _ => unreachable!()
    }
}