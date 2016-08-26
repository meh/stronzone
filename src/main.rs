use std::net::UdpSocket;
use std::{mem, slice};

extern crate ws281x;

extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("stronzone")
		.version("0.1.0")
		.author("meh. <meh@schizofreni.co>")
		.about("Stronzone con le lucette in avvicinamento.")
		.arg(Arg::with_name("ADDRESS")
			.takes_value(true)
			.index(1)
			.required(true)
			.help("The address to bind to."))
		.arg(Arg::with_name("PORT")
			.takes_value(true)
			.index(2)
			.required(true)
			.help("The port to bind to."))
		.get_matches();

	let mut handle = ws281x::handle::new()
		.dma(5)
		.channel(0, ws281x::channel::new()
			.pin(18)
			.count(8 * 8)
			.brightness(55)
			.build().unwrap())
		.build().unwrap();

	let socket = UdpSocket::bind((
		matches.value_of("ADDRESS").unwrap(),
		matches.value_of("PORT").unwrap().parse().unwrap()
	)).unwrap();

	let mut buffer = [0; 512];

	while let Ok((size, _source)) = socket.recv_from(&mut buffer) {
		// brightness: i32, leds: [u32; 8 * 8]
		if size != mem::size_of::<i32>() + (8 * 8 * mem::size_of::<ws281x::Led>()) {
			continue;
		}

		let values = unsafe {
			slice::from_raw_parts(buffer.as_ptr() as *const ws281x::Led, 1 + 8 * 8)
		};

		handle.channel_mut(0).set_brightness(values[0] as i32);
		for (src, dst) in values.iter().cloned().skip(1).zip(handle.channel_mut(0).leds_mut().iter_mut()) {
			*dst = src
		}

		handle.render().unwrap();
		handle.wait().unwrap();
	}
}
