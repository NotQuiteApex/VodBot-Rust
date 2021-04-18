// Helper stuff for Twitch API

use reqwest::blocking::Client;


struct Channel<'a> {
	id: &'a str,
	login: &'a str,
	display_name: &'a str,

	created_at: &'a str,
}

struct VodData<'a> {
	id: &'a str,
	title: &'a str,
	created_at: &'a str,
	duration: &'a str,

	streamer_id: &'a str,
	streamer_name: &'a str,
}

struct ClipData<'a> {
	id: &'a str,
	title: &'a str,
	created_at: &'a str,
	view_count: u64,

	streamer_id: &'a str,
	streamer_name: &'a str,

	clipper_id: &'a str,
	clipper_name: &'a str,
}


pub fn get_access_token(client: &Client, client_id: &String, client_secret: &String) -> String {
	let url = format!(
		"https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials",
		client_id,
		client_secret,
	);

	let res = client.post(url).send();

	if let Ok(response) = res {
		if let Ok(text) = response.text() {
			let parse: serde_json::Result<serde_json::Value> = serde_json::from_str(&text);
			if let Ok(mut json) = parse {
				serde_json::from_value(json["access_token"].take())
					.expect("Cannot read key \"access_token\" from response from Twitch for auth.")
			} else {
				panic!("Cannot parse response as JSON from Twitch for auth.");
			}
		} else {
			panic!("Cannot read response from Twitch for auth.");
		}
	} else {
		panic!("No response from Twitch for auth.");
	}
}