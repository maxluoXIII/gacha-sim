use clap::{App, Arg, ArgGroup};
use rand::Rng;
use std::cmp;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
struct ArgumentError {
    details: String,
}

impl ArgumentError {
    fn new(msg: &str) -> ArgumentError {
        ArgumentError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ArgumentError {}

pub struct AppConfig {
    pub rolls: usize,
    pub num_simulations: usize,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, Box<dyn Error>> {
        let matches = App::new("Gacha Simulator")
            .version("0.1.0")
            .author("Maxiwell Luo <maxluoxiii@gmail.com>")
            .about("Run simulations for Arknights limited banner")
            .arg(
                Arg::with_name("sims")
                    .short("n")
                    .long("sims")
                    .value_name("NUM_SIMULATIONS")
                    .default_value("10000")
                    .help("Provides the number of simulations to run, default 10000"),
            )
            .arg(
                Arg::with_name("rolls")
                    .short("r")
                    .long("rolls")
                    .value_names(&["ROLLS"])
                    .help("Provides number of rolls for which to calculate probability"),
            )
            .arg(
                Arg::with_name("resources")
                    .short("e")
                    .long("resources")
                    .value_names(&["TEN-ROLL", "SINGLE-ROLL", "ORUNDUM"])
                    .help("Provides number of rolls through number of resources"),
            )
            .group(ArgGroup::with_name("set-rolls").args(&["rolls", "resources"]))
            .get_matches();

        let num_sims = matches.value_of("sims").unwrap().parse::<usize>()?;

        if let Some(rolls) = matches.value_of("rolls") {
            return Ok(AppConfig {
                rolls: rolls.parse::<usize>()?,
                num_simulations: num_sims,
            });
        } else if let Some(resources) = matches.values_of("resources") {
            let rolls = get_rolls(resources)?;
            return Ok(AppConfig {
                rolls: rolls,
                num_simulations: num_sims,
            });
        }

        return Err(
            ArgumentError::new("Failed to parse arguments, this should never happen").into(),
        );
    }
}

fn get_rolls(mut resources: clap::Values) -> Result<usize, ParseIntError> {
    let ten_ticket_rolls = resources.next().unwrap().parse::<usize>()?;
    let single_ticket_rolls = resources.next().unwrap().parse::<usize>()?;
    let orundum_rolls = resources.next().unwrap().parse::<usize>()?;

    Ok((ten_ticket_rolls * 10) + single_ticket_rolls + (orundum_rolls / 600))
}

pub fn run(configs: AppConfig) {
    // Only using the values from idx 1..300
    let mut results = [0; 301];

    for _ in 0..configs.num_simulations {
        run_simulation(&mut results);
    }

    println!("{:?}", results);

    let mut cum_prob = 0f64;
    for i in 1..=configs.rolls {
        cum_prob += f64::from(results[i]) / (configs.num_simulations as f64);
    }
    // let cum_prob = f64::from(cum_prob) / (configs.num_simulations as f64);
    println!(
        "The probability of getting the targeted operator is {}",
        cum_prob
    );
}

fn run_simulation(results: &mut [i32]) {
    let mut got_targeted_op = false;

    let num_six_star_rate_ups = 2f64;
    let num_targeted_rate_ups = 1f64;
    let target_ratio = num_targeted_rate_ups / num_six_star_rate_ups;
    let mut rolls_since_last_six_star = 0;
    for roll_num in 1..=300 {
        let rate = six_star_rate(rolls_since_last_six_star);
        if rand::thread_rng().gen_range(1..=100) <= rate {
            if rand::thread_rng().gen::<f64>() < target_ratio * 0.7 {
                results[roll_num] += 1;
                got_targeted_op = true;
                break;
            }
            rolls_since_last_six_star = 0;
        } else {
            rolls_since_last_six_star += 1;
        }
    }
    if !got_targeted_op {
        results[300] += 1;
    }
}

fn six_star_rate(rolls_since_last_six_star: i32) -> i32 {
    2 + cmp::max((rolls_since_last_six_star - 50) * 2, 0)
}
