// VodBot (c) 2020-21 Logan "NotQuiteApex" Hickok-Dickson

use std::path::Path;

extern crate clap;
extern crate dirs;
extern crate serde_json;
extern crate ansi_term;

use ansi_term::Color::{Red, Yellow, Purple, White};
use clap::{Arg, App, SubCommand};

mod util;
mod twitch;
mod commands {
	pub mod pull;
	pub mod stage;
	pub mod upload;
}


fn deffered_main() -> Result<(), util::ExitMsg> {
	// Load the environment variables from cargo for this info.
	const AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
	const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

	// Get the .vodbot dir.
	// Error if the home directory does not exist.
	if dirs::home_dir().is_none() {
		return Err(util::ExitMsg{
			code: util::ExitCode::CannotCreateDir,
			msg: format!("Cannot locate home directory.")
		})
	}

	// Create base directory.
	let mut vodbot_dir = dirs::home_dir().unwrap(); vodbot_dir.push(".vodbot");
	util::create_dir(&vodbot_dir)?;

	// Run the argument parser.
	let matches = App::new("VodBot")
		.author(AUTHORS.unwrap_or("AUTHORS UNKNOWN"))
		.version(VERSION.unwrap_or("VERSION UNKNOWN"))
		.about("A VOD and Clip Manager for Twitch.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.value_name("FILE")
			.help("Sets the location of the config file.")
			.takes_value(true))
		// Pull command, downloads VODs or Clips (or both).
		.subcommand(SubCommand::with_name("pull")
			.about("Pulls VODs or Clips from Twitch")
			.arg(Arg::with_name("type")
				.possible_values(&["vods", "clips", "both"])
				.help("Type of content to pull.")))
		// Stage command, for staging VODs/Clips for slicing/uploading.
		.subcommand(SubCommand::with_name("stage")
			.about("Stages VODs and Clips for slicing/uploading")
			// Adding a new stage.
			.subcommand(SubCommand::with_name("add")
				.about("Add a stage with a VOD or Clip ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the VOD or Clip to stage.")))
			// Editing an existing stage.
			.subcommand(SubCommand::with_name("edit")
				.about("Edit an existing stage with its ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the stage to edit.")))
			// Listing stages or getting details of a specific stage.
			.subcommand(SubCommand::with_name("list")
				.about("List current stages, or display info of a specific stage with an ID.")
				.arg(Arg::with_name("id")
					.help("ID of the stage to view. Optional.")))
			// Removing an existing stage.
			.subcommand(SubCommand::with_name("rm")
				.about("Remove an existing stage with an ID.")
				.arg(Arg::with_name("id")
					.required(true)
					.help("ID of the stage to remove."))))
		// Upload command, for uploading staged data to YouTube.
		.subcommand(SubCommand::with_name("upload")
			.about("Uploads stages to YouTube")
			.arg(Arg::with_name("id")
				.required(true)
				.help("ID of the stage to upload; \"all\" to upload all stages; \
						\"logout\" to logout of the YouTube account.")))
		// All done, parse input and get matches.
		.get_matches();
	
	println!("{}", Purple.bold().paint(format!(
			"* VodBot {} (c) 2020-21 {} *", 
			VERSION.unwrap_or("VERSION UNKNOWN"), 
			AUTHORS.unwrap_or("AUTHORS UNKNOWN")
		))
	);
	
	// Load config.
	let mut default_config_path = vodbot_dir.clone();
	default_config_path.push("conf.json");

	print!("{}", White.dimmed().paint("Loading config... "));
	let config_path = Path::new( matches.value_of("config")
		.unwrap_or(default_config_path.to_str().unwrap()) );
	
	if !config_path.exists() {
		println!("{}", White.dimmed().paint("Could not find config, attempting to create one..."));
		util::create_conf(config_path)?;
	}

	let config: util::Config = util::load_conf(config_path)?;
	
	// Run the commands
	if let Some(matches) = matches.subcommand_matches("pull") {
		commands::pull::run(matches, config)?
	} else if let Some(matches) = matches.subcommand_matches("stage") {
		commands::stage::run(matches, config)?
	} else if let Some(matches) = matches.subcommand_matches("upload") {
		commands::upload::run(matches, config)?
	}

	Ok(())
}


fn main() {
	std::process::exit(match deffered_main() {
		Ok(()) => 0,
		Err(err) => {
			println!("\n{} ({}) {}",
				Red.bold().paint("Error!"),
				Yellow.bold().paint((err.code as u32).to_string()),
				Red.bold().paint(err.msg)
			);
			err.code as i32
		}
	});
}
