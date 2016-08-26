use std::thread;
use std::time::Duration;

extern crate stronzone;

fn main() {
	let mut canvas = stronzone::canvas()
		.add((0, 0), stronzone::grid::new()
			.brightness(55)
			.connect(("192.168.0.145", 9001)).unwrap());

	let mut check = 0;
	loop {
		for x in 0 .. canvas.width() {
			for y in 0 .. canvas.height() {
				if (check == 0 && x % 2 == y % 2) || (check == 1 && x % 2 != y % 2) {
					canvas.set((x, y), stronzone::Led::new(0.0, 0.0, 0.0));
				}
				else {
					canvas.set((x, y), stronzone::Led::new(1.0, 1.0, 1.0));
				}
			}
		}

		canvas.sync().unwrap();

		thread::sleep(Duration::from_millis(500));
		check = if check == 0 { 1 } else { 0 };
	}
}
