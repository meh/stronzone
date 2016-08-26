extern crate stronzone;

fn main() {
	let mut canvas = stronzone::canvas()
		.add((0, 0), stronzone::grid::new()
			.brightness(0)
			.connect(("192.168.0.145", 9001)).unwrap());

	canvas.clear();
	canvas.sync().unwrap();
}
