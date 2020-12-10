mod position;

pub use position::*;

use hecs::EntityBuilder;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConstructError {}
pub trait Construct {
    fn construct_on_to<'c, 'eb>(
        &'c self,
        builder: &'eb mut EntityBuilder,
    ) -> Result<&'eb mut EntityBuilder, ConstructError>;
}

pub trait EntityBuilderExtension: Sized {
    fn try_construct<'c, 'eb, C: Construct>(
        &'eb mut self,
        constructor: &'c C,
    ) -> Result<&mut Self, ConstructError>;
    fn construct<'c, 'eb, C: Construct>(&'eb mut self, constructor: &'c C) -> &'eb mut Self {
        self.try_construct(constructor).unwrap()
    }
}

impl EntityBuilderExtension for EntityBuilder {
    fn try_construct<'c, 'eb, C: Construct>(
        &'eb mut self,
        constructor: &'c C,
    ) -> Result<&'eb mut Self, ConstructError> {
        constructor.construct_on_to(self)
    }
}
