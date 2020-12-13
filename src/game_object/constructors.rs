mod inspect;
mod position;

use super::state::{Position, Render};
use crate::typedefs::collision;
use enum_dispatch::*;
use hecs::EntityBuilder;
use imgui::Ui;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

pub use inspect::Inspect;
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
impl<Tag: ConstructTag> Inspect for Tag {
    fn inspect_mut(&mut self, _: &Ui<'_>) {}
}

/// Each variant represents what context needs to be provided to the constructor.
#[enum_dispatch(Inspect)]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Constructor {
    Contextless(ContextlessConstructor),
    Position(PositionConstructor),
}

impl TryAsRef<ContextlessConstructor> for Constructor {
    fn try_as_ref(&self) -> Option<&ContextlessConstructor> {
        if let Constructor::Contextless(ref value) = self {
            Some(value)
        } else {
            None
        }
    }
}

impl TryAsRef<PositionConstructor> for Constructor {
    fn try_as_ref(&self) -> Option<&PositionConstructor> {
        if let Constructor::Position(ref value) = self {
            Some(value)
        } else {
            None
        }
    }
}
impl Display for Constructor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contextless(value) => value.fmt(f),
            Self::Position(value) => value.fmt(f),
        }
    }
}

impl Constructor {
    pub fn iter() -> impl Iterator<Item = Constructor> {
        ContextlessConstructor::iter()
            .map(Constructor::from)
            .chain(PositionConstructor::iter().map(Constructor::from))
    }
}

pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}

macro_rules! construct_enum_impl {
    (enum $name:ident<Context = $context:ty> { $($variant:tt,)+ }) => {
        #[enum_dispatch(Inspect)]
        #[derive(Serialize, Deserialize, Clone, EnumIter, Display, Eq, PartialEq)]
        pub enum $name {
            $($variant($variant)),+
        }

        $(
            impl TryInto<$variant> for Constructor {
                type Error = &'static str;
                fn try_into(self) -> Result<$variant, Self::Error> {
                    let value: $name = self.try_into()?;
                    value.try_into()
                }
            }
            impl TryAsRef<$variant> for Constructor {
                 fn try_as_ref(&self) -> Option<&$variant> {
                    let value: &$name = self.try_as_ref()?;
                    match value {
                        $name::$variant(ref value) => Some(value),
                        _ => None,
                    }
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

construct_enum_impl!(
    enum ContextlessConstructor<Context = ()> {
        Render,
    }
);
construct_enum_impl!(
    enum PositionConstructor<Context = collision::Vec2> {
        Position,
    }
);

// TODO: how to do context?????
// what about character specific context
// context is an associated type, two levels of enums are provided, one that tells requested context, and the next that is just enum_dispatch
