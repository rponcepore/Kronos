//! environment.rs 

// This is the file for stating and configuring the environment based on command line arguments.

use::clap::Parser;

#[derive(Debug)]
pub enum BuildType {
    Dev,
    Prod,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidDevProdArgument,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Kargs{
    // Build type
    #[arg(short, long, required(false), default_value("dev"))]
    build: String,

    #[arg(long, required(false), default_value("todo"))]
    server_address: String,

    #[arg(long, required(false), default_value("todo"))]
    server_port: String,

    #[arg(long, required(false), default_value("todo"))]
    db_url: String,

    #[arg(long, required(false), default_value("todo"))]
    db_name: String,
}

#[derive(Debug)]
pub struct Config {
    pub build: BuildType,
    pub server_address: String,
    //database_url: String,
    //database_name: String,
}

impl Config{
    pub fn new(build: BuildType) -> Self {
        match build {
            BuildType::Dev => {
                Self {  build : build,
                        server_address : "127.0.0.1:8000".to_string(), //this actually might need to become 0.0.0.0:8000 for dockerization
                        //link: 
                        //database_name : "TODO".to_string(),
                        //database_url : "TODO".to_string(),
                    }
            },
            BuildType::Prod => {
                todo ! ()
            },
        }
    }
}


/*
 * The command-line argument parser. This defines a whole series of environment
 * variables that we need to run the thing in dev or prod environments.
 * */
pub fn parse_args(args: Kargs) -> Result<Box<Config>, ParseError> {
    // needs to be mut for future changes based on the parsing
    let config= match args.build.to_lowercase().as_str() {
        "dev" =>  Config::new(BuildType::Dev),
        "prod" => Config::new(BuildType::Prod),
        _ => return Err(ParseError::InvalidDevProdArgument),
    };
    
    // Adjust config as needed based on the other command line arguments. 

    // Return the correct config to the main method to start constructing things.
    Ok(Box::new(config))
}