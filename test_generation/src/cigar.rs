use rand::{Rng, thread_rng};

#[derive(Copy, Clone, Debug)]
pub enum CIGAREntry {
	M,
	I,
	S,
	Eq,
	X
}

impl CIGAREntry {
	pub fn generate_random() -> Self {
		let value = thread_rng().gen_range(0..=2);

		match value {
			0 => {
				Self::M
			},
			1 => {
				Self::Eq
			},
			2 => {
				Self::X
			}
			_ => {
				panic!()
			}
		}
	}

	pub fn to_u8(self) -> u8 {
		match self {
			CIGAREntry::M => {
				0x4D
			}
			CIGAREntry::I => {
				0x49
			}
			CIGAREntry::S => {
				0x53
			}
			CIGAREntry::Eq => {
				0x3D
			}
			CIGAREntry::X => {
				0x58
			}
		}
	}
}
