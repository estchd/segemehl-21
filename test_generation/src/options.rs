#[derive(Debug, Clone)]
pub struct Options {

}

impl From<&str> for Options {
	fn from(_: &str) -> Self {
		Self {}
	}
}