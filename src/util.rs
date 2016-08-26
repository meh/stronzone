use regex::Regex;
use {Led, grid, Grid};

lazy_static! {
	static ref AT:      Regex = Regex::new(r"\{(\d+) (\d+)\}").unwrap();
	static ref COORDS:  Regex = Regex::new(r"\[(\w+)\]").unwrap();
	static ref ADDR:    Regex = Regex::new(r"(?:[\]}]|\A)\s*([^\]}]*)$").unwrap();
	static ref HEX_RGB: Regex = Regex::new(r"#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})").unwrap();
}

pub fn grid(def: &str, brightness: i32) -> ((u32, u32), Grid) {
	let at = if let Some(caps) = AT.captures(def) {
		(caps.at(1).unwrap().parse().unwrap(), caps.at(2).unwrap().parse().unwrap())
	}
	else {
		(0, 0)
	};

	let mut grid = grid::new();
	grid.brightness(brightness);

	if let Some(caps) = COORDS.captures(def) {
		grid.coordinates(match caps.at(1) {
			Some("DOWN") => grid::DOWN,
			_            => grid::UP,
		});
	}

	(at, grid.connect(ADDR.captures(def).unwrap().at(1).unwrap()).unwrap())
}

pub fn color(value: &str) -> Option<Led> {
	HEX_RGB.captures(value.as_ref()).map(|captures| {
		Led::new(
			u8::from_str_radix(captures.at(1).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
			u8::from_str_radix(captures.at(2).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
			u8::from_str_radix(captures.at(3).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
		)
	})
}
