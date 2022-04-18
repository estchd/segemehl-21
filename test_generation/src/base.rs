use rand::{Rng, thread_rng};

#[derive(Copy, Clone, Debug)]
pub enum Base {
	A,
	G,
	C,
	T
}

impl Base {
	pub fn generate_random() -> Self {
		let mut rand = thread_rng();
		let value = rand.gen_range(0..4);
		match value {
			0 => {
				Self::A
			},
			1 => {
				Self::G
			},
			2 => {
				Self::C
			},
			3 => {
				Self::T
			}
			_ => {
				panic!()
			}
		}
	}

	pub fn to_u8(self) -> u8{
		match self {
			Base::A => {
				0x41
			}
			Base::G => {
				0x47
			}
			Base::C => {
				0x43
			}
			Base::T => {
				0x54
			}
		}
	}
}
