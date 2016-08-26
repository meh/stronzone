use std::fs::File;
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

extern crate stronzone;
extern crate bdf;

#[macro_use]
extern crate lazy_static;

extern crate regex;
use regex::Regex;

extern crate clap;
use clap::{Arg, App};

lazy_static! {
	static ref AT:      Regex = Regex::new(r"\{(\d+) (\d+)\}").unwrap();
	static ref COORDS:  Regex = Regex::new(r"\[(\w+)\]").unwrap();
	static ref ADDR:    Regex = Regex::new(r"[\]}]\s*([^\]}]*)$").unwrap();
	static ref HEX_RGB: Regex = Regex::new(r"#([:xdigit:]{2})([:xdigit:]{2})([:xdigit:]{2})").unwrap();
}

fn grid(def: &str, brightness: i32) -> ((u32, u32), stronzone::Grid) {
	let at = if let Some(caps) = AT.captures(def) {
		(caps.at(1).unwrap().parse().unwrap(), caps.at(2).unwrap().parse().unwrap())
	}
	else {
		(0, 0)
	};

	let mut grid = stronzone::grid::new();
	grid.brightness(brightness);

	if let Some(caps) = COORDS.captures(def) {
		grid.coordinates(match caps.at(1) {
			Some("DOWN") => stronzone::grid::DOWN,
			_            => stronzone::grid::UP,
		});
	}

	(at, grid.connect(ADDR.captures(def).unwrap().at(1).unwrap()).unwrap())
}

fn color(value: &str) -> Option<stronzone::Led> {
	HEX_RGB.captures(value.as_ref()).map(|captures| {
		stronzone::Led::new(
			u8::from_str_radix(captures.at(1).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
			u8::from_str_radix(captures.at(2).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
			u8::from_str_radix(captures.at(3).unwrap_or("0"), 16).unwrap_or(0) as f32 / 255.0,
		)
	})
}

fn main() {
	let matches = App::new("text")
		.version("0.1.0")
		.author("meh. <meh@schizofreni.co>")
		.about("Draw sliding gradient text.")
		.arg(Arg::with_name("time")
			.short("t")
			.long("time")
			.takes_value(true)
			.help("Time to sleep in milliseconds between each slide (defaultt is 100)."))
		.arg(Arg::with_name("brightness")
			.short("b")
			.long("brightness")
			.takes_value(true)
			.help("The led brightness (default is 55)."))
		.arg(Arg::with_name("grid")
			.short("g")
			.long("grid")
			.takes_value(true)
			.multiple(true)
			.required(true)
			.help("Add a grid to the canvas."))
		.arg(Arg::with_name("font")
			.short("f")
			.long("font")
			.takes_value(true)
			.required(true)
			.help("Path to a BDF font to use for rendering."))
		.arg(Arg::with_name("y")
			.short("y")
			.long("y")
			.takes_value(true)
			.help("The Y offset withing the glyph."))
		.arg(Arg::with_name("color")
			.short("c")
			.long("color")
			.takes_value(true)
			.multiple(true)
			.help("The colors in the gradient (default is #ffffff)."))
		.arg(Arg::with_name("TEXT")
			.index(1)
			.required(true)
			.help("The text to draw."))
		.get_matches();

	let time       = matches.value_of("time").unwrap_or("100").parse().unwrap();
	let brightness = matches.value_of("brightness").unwrap_or("55").parse().unwrap();
	let y_offset   = matches.value_of("y").unwrap_or("0").parse::<u32>().unwrap();
	let font       = stronzone::font::read(File::open(matches.value_of("font").unwrap()).unwrap()).unwrap();

	// Create the canvas based on definition.
	let mut canvas = matches.values_of("grid").unwrap().fold(stronzone::canvas(), |canvas, def| {
		let (at, grid) = grid(def, brightness);
		canvas.add(at, grid)
	});

	// The text as single characters surrounded by spaces.
	let mut text = VecDeque::new();
	text.push_back(' ');
	for ch in matches.value_of("TEXT").unwrap().chars() {
		text.push_back(ch);
	}
	text.push_back(' ');

	// The rainbow colors to create the gradient.
	let mut colors = matches.values_of("color").map(|v| v.collect()).unwrap_or(vec!["#ffffff"]).iter()
		.map(|c| color(c).unwrap()).collect::<VecDeque<_>>();

	// The current offset within the first glyph.
	let mut offset = 0;

	loop {
		let gradient = stronzone::color::Gradient::new(colors.iter().cloned());

		for (x, led) in (0 .. canvas.width()).zip(gradient.take(canvas.width() as usize)) {
			let xx = x + offset;

			if let Some(glyph) = text.get((xx / font.bounds().width) as usize).and_then(|ch| font.glyphs().get(ch)) {
				let map = glyph.map();
				let xx  = xx % font.bounds().width;

				for y in 0 .. canvas.height() {
					if y < font.bounds().height && xx < font.bounds().width && map.get(xx, y + y_offset) {
						canvas.set((x, y), led);
					}
				}
			}
		}

		// Synchronize the canvas and clear it for the next rendering.
		canvas.sync().unwrap();
		canvas.clear();

		thread::sleep(Duration::from_millis(time));

		// Shift text or offset.
		if offset >= font.bounds().width {
			let ch = text.pop_front().unwrap();
			text.push_back(ch);
			offset = 1;
		}
		else {
			offset += 1;
		}

		// Shift colors by one.
		{
			let color = colors.pop_front().unwrap();
			colors.push_back(color);
		}
	}
}
