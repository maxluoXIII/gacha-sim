use gacha_sim::AppConfig;

fn main() {
    let configs = AppConfig::new();
    match configs {
        Ok(configs) => {
            println!("You have {} rolls!", configs.rolls);
            gacha_sim::run(configs);
        },
        Err(error) => {
            println!("Error: {}", error.to_string());
            std::process::exit(1);
        },
    }
}
