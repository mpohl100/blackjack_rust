//use clap::{App, Arg, value_t};
pub use crate::blackjack::blackjack_configuration::PlayConfiguration;
pub use crate::blackjack::blackjack_configuration::StrategyConfiguration;


fn validate_positive_integer(value: String) -> Result<(), String> {
    match value.parse::<u64>() {
        Ok(number) if number > 0 => Ok(()),
        _ => Err("Invalid positive integer.".to_string()),
    }
}

pub fn get_commandline_params<'a>(binary_name: String, description: &'a str) -> clap::App<'a, 'a> 
{
    // Define the command line interface using Clap
    let matches = clap::App::new(binary_name)
    .version("1.0")
    .author("Michael Pohl")
    .about(description)
    .arg(
        clap::Arg::with_name("number_hands")
            .short("n")
            .long("number-hands")
            .value_name("N")
            .help("the number of hands to play")
            .validator(validate_positive_integer)
            .takes_value(true),
    ).arg(
        clap::Arg::with_name("number_threads")
            .short("t")
            .long("number-threads")
            .value_name("T")
            .help("the number of threads for the strat calculation")
            .validator(validate_positive_integer)
            .takes_value(true),
    );
    matches
}

fn get_number_hands(app: clap::App<'_,'_>) -> u64
{
    let matches = app.get_matches(); 
    let _number_str = matches.value_of("number_hands");
    if let Some(_number) = matches.value_of("number_hands") {
        let parsed_number: u64 = clap::value_t!(matches.value_of("number_hands"), u64).unwrap();
        parsed_number
    }
    else{
        // default is one million hands
        1000000
    }
}

pub fn get_play_config(app: clap::App<'_,'_>) -> PlayConfiguration
{
    let play_config = PlayConfiguration{nb_hands: get_number_hands(app).try_into().unwrap(), play_normal: true};
    play_config
}

fn get_number_threads(app: clap::App<'_,'_>) -> u64
{
    let matches = app.get_matches(); 
    let _number_str = matches.value_of("number_threads");
    if let Some(_number) = matches.value_of("number_threads") {
        let parsed_number: u64 = clap::value_t!(matches.value_of("number_threads"), u64).unwrap();
        parsed_number
    }
    else{
        // default is zero threads use the sequential version
        0
    }
}

pub fn get_strat_config(app: clap::App<'_,'_>) -> StrategyConfiguration
{
    let strat_config = StrategyConfiguration{nb_threads: get_number_threads(app).try_into().unwrap()};
    strat_config
}