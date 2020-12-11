mod inspect;
mod position;

use super::state::{Position, Render};
use crate::typedefs::collision;
use enum_dispatch::*;
use hecs::EntityBuilder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

pub use position::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConstructError {
    MissingRequiredComponent,
}
pub trait Construct {
    type Context;
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError>;
}

pub trait ConstructTag: Default {}

impl<Tag: ConstructTag + hecs::Component> Construct for Tag {
    type Context = ();
    fn construct_on_to<'builder>(
        &self,
        builder: &'builder mut EntityBuilder,
        _: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(Self::default()))
    }
}

/// Each variant represents what context needs to be provided to the constructor.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Constructor {
    Contextless(ContextlessConstructor),
    Position(PositionConstructor),
}

impl Display for Constructor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contextless(value) => value.fmt(f),
            Self::Position(value) => value.fmt(f),
        }
    }
}

impl From<ContextlessConstructor> for Constructor {
    fn from(value: ContextlessConstructor) -> Self {
        Self::Contextless(value)
    }
}
impl From<PositionConstructor> for Constructor {
    fn from(value: PositionConstructor) -> Self {
        Self::Position(value)
    }
}

impl Constructor {
    pub fn iter() -> impl Iterator<Item = Constructor> {
        ContextlessConstructor::iter()
            .map(Constructor::from)
            .chain(PositionConstructor::iter().map(Constructor::from))
    }
}

macro_rules! construct_variant {
    ($name:ident; $context:ty; $($variant:tt),+) => {
        #[enum_dispatch]
        #[derive(Serialize, Deserialize, Clone, EnumIter, Display, Eq, PartialEq)]
        pub enum $name {
            $($variant($variant)),+
        }
        $(
            impl From<$variant> for $name {
                fn from(value: $variant) -> Self {
                    Self::$variant(value)
                }
            }
        )+

        impl Construct for $name {
            type Context = $context;
            fn construct_on_to<'constructor, 'builder>(
                &'constructor self,
                builder: &'builder mut EntityBuilder,
                context: Self::Context,
            ) -> Result<&'builder mut EntityBuilder, ConstructError> {
                match self {
                    $(
                        Self::$variant(ref value) => value.construct_on_to(builder, context),
                    )+
                }
            }
        }

    };
}

construct_variant!(ContextlessConstructor; (); Render);
construct_variant!(PositionConstructor; collision::Vec2; Position);

// TODO: how to do context?????
// what about character specific context
// context is an associated type, two levels of enums are provided, one that tells requested context, and the next that is just enum_dispatch
