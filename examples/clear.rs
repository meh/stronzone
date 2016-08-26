extern crate stronzone;

extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("text")
		.version("0.1.0")
		.author("meh. <meh@schizofreni.co>")
		.about("Clear and turn off all leds.")
		.arg(Arg::with_name("grid")
			.short("g")
			.long("grid")
			.takes_value(true)
			.multiple(true)
			.required(true)
			.help("Add a grid to the canvas."))
		.get_matches();

	// Create the canvas based on definition.
	let mut canvas = matches.values_of("grid").unwrap().fold(stronzone::canvas(), |canvas, def| {
		let (at, grid) = stronzone::util::grid(def, 0);
		canvas.add(at, grid)
	});

	canvas.clear();
	canvas.sync().unwrap();
}
