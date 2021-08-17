use crate::transformers::Transformer;

pub trait TransformerConstructor {
	fn get_constructable_transformers(&self) -> Vec<String>;

	fn construct_transformer(&self, name: String) -> Box<dyn Transformer>;
}