// Utility Functions and Types

extern crate serde_json;

use std::path::Path;
use std::fs;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub enum ExitCode {
	_CleanExit,
	MissingFromConfig,
	CannotCreateDir,
	NoConnection,
	CannotParseResponse,
	CannotFindAccessToken,
}

pub struct ExitMsg {
	pub msg: String,
	pub code: ExitCode,
}


const DEFAULT_CONFIG: &str = r#"
{
	"twitch_channels": [ "46moura",
		"alkana", "batkigu", "hylianswordsman1"
		"juicibit", "michiri9", "notquiteapex",
		"pissyellowcrocs", "percy_creates", "voobo",
	],
	
	"twitch_client_id": "[[INSERT CLIENT ID HERE]]",
	"twitch_client_secret": "[[INSERT CLIENT SECRET HERE]]",
	
	"stage_timezone": "US/Eastern",
	"stage_format": {
		"watch": "-- Watch live at {links}",
		"discord": "-- Join the Discord https://discord.gg/v2t6uag",
		"credits": "\n{watch}\n{discord}"
	},
	
	"youtube_client_path": "{vodbot}/yt-client.json",
	"youtube_pickle_path": "{vodbot}/yt-pickle.pikl",

	"vod_dir": "{vodbot}/vods",
	"clip_dir": "{vodbot}/clips",
	"temp_dir": "{vodbot}/temp",
	"stage_dir": "{vodbot}/stage",
}
"#;


pub fn create_conf(conf_path: &Path) {
	fs::write(conf_path, DEFAULT_CONFIG)
		.expect(format!("Cannot write to config file at {}.", conf_path.display()).as_str());
	
	panic!("Created config at {}, please edit it to continue. Exiting...", conf_path.display())
}


pub fn load_conf(conf_path: &Path) -> serde_json::Value {
	let config_file = match fs::File::open(conf_path) {
		Err(why) => panic!("Cannot open config file. {}", why),
		Ok(file) => file
	};
	let reader = BufReader::new(config_file);
	let conf: serde_json::Value = serde_json::from_reader(reader)
		.expect("Could not parse config.");

	// TODO: Do some checks for values.

	conf
}


pub fn load_string_config(conf: &mut serde_json::Value, key: &str) -> Result<String, ExitMsg> {
	if let Ok(j) = serde_json::from_value(conf[key].take()) {
		Ok(j)
	} else {
		return Err( ExitMsg{
			code: ExitCode::MissingFromConfig,
			msg: format!("Cannot load key `{}` from config as a string.", key)
		});
	}
}


pub fn load_list_config(conf: &mut serde_json::Value, key: &str) -> Result<Vec<String>, ExitMsg> {
	if let Ok(j) = serde_json::from_value(conf[key].take()) {
		Ok(j)
	} else {
		return Err( ExitMsg{
			code: ExitCode::MissingFromConfig,
			msg: format!("Cannot load key `{}` from config as a string.", key)
		});
	}
}


pub fn create_dir(dir_path: &Path) -> Result<(), ExitMsg> {
	match fs::create_dir_all(&dir_path) {
		Err(why) => {
			return Err(ExitMsg{
				code: ExitCode::CannotCreateDir,
				msg: format!("Cannot create directory `{}`, reason: \"{}\".", 
					&dir_path.display(), why)
			})
		},
		_ => Ok(())
	}
}
