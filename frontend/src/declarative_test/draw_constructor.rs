use crate::declarative_test::draw::Draw;

pub trait DrawConstructor {
	fn get_constructable_draws(&self) -> Vec<String>;

	fn construct_draw(&self, name: String) -> Box<dyn Draw>;
}