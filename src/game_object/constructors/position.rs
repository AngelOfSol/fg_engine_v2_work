use super::{inspect::Inspect, Construct, ConstructError};
use crate::imgui_extra::UiExtensions;
use crate::{game_object::state::Position, typedefs::collision};
use hecs::EntityBuilder;
use imgui::*;

impl Inspect for Position {
    fn inspect_mut(&mut self, ui: &Ui<'_>) {
        ui.input_vec2_pixels(im_str!("Offset"), &mut self.value);
    }
}

impl Construct for Position {
    type Context = collision::Vec2;
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        offset: collision::Vec2,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(Self {
            value: self.value + offset,
        }))
    }
}
