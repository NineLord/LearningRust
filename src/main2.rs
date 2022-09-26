const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

trait Monster {
	fn cookie(self: &mut Self);
	fn cookie2(&self);
	fn cookie3(&mut self);
	fn cookie4(self);
}

struct Shimi {
	food: i32
}

impl Monster for Shimi {

	fn cookie(self: &mut Self) {
		println!("cookie! {}", self.food);
	}

	fn cookie2(&self) {
		println!("cookie2! {}", self.food);
	}

	fn cookie3(&mut self) {
		println!("cookie3! {}", self.food);
	}

	fn cookie4(self) {
		println!("cookie4! {}", self.food);
	}
}

impl Shimi {
	fn cookie5(this: &Self) {
		println!("cookie5! {}", this.food);
	}
}

fn main() {
	// let mut missiles: i32 = STARTING_MISSILES;
	// let ready: i32 = READY_AMOUNT;
	let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
	println!("Firing {} of my {} missiles...", ready, missiles);
	// missiles = missiles - ready;
	print!("{} missiles left\n", missiles - ready);
	// READY_AMOUNT = 1;

	let mut shimi = Shimi{food:23};
	// shimi.cookie5();
	Shimi::cookie5(&shimi);
	shimi.cookie();
	shimi.cookie2();
	shimi.cookie3();
	shimi.cookie4();
	// shimi.cookie2();
}
