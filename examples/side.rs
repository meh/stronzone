extern crate stronzone;

fn main() {
	let mut canvas = stronzone::canvas()
		.add((0, 0), stronzone::grid::new()
			.brightness(55)
			.connect(("192.168.0.145", 9001)).unwrap());

	let width = canvas.width();
	let height = canvas.height();

	for x in 0 .. width {
		// Red horizontal strip on top.
		canvas.set((x, 0), stronzone::Led::new(1.0, 0.0, 0.0));

		// Blue horizontal strip on bottom.
		canvas.set((x, height - 1), stronzone::Led::new(0.0, 0.0, 1.0));
	}

	for y in 0 .. height {
		// Yellow vertical strip on middle left.
		canvas.set((3, y), stronzone::Led::new(1.0, 1.0, 0.0));

		// Green vertical strip on middle right.
		canvas.set((4, y), stronzone::Led::new(0.0, 1.0, 0.0));
	}

	canvas.sync().unwrap();
}
