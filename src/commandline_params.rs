//use clap::{App, Arg, value_t};

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
    );
    matches
}

pub fn get_number_hands(app: clap::App<'_,'_>) -> u64
{
    let matches = app.get_matches(); 
    let number_str = matches.value_of("number_hands");
    if let Some(_number) = matches.value_of("number_hands") {
        let parsed_number: u64 = clap::value_t!(matches.value_of("number_hands"), u64).unwrap();
        parsed_number
    }
    else{
        // default is one million hands
        1000000
    }
}
