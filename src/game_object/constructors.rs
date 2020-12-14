mod inspect;
mod position;

use super::state::{ExpiresAfterAnimation, Position, Timer};
use crate::{
    character::state::components::GlobalGraphic, imgui_extra::UiExtensions, roster::YuyukoGraphic,
    typedefs::collision,
};
use enum_dispatch::*;
use hecs::EntityBuilder;
use imgui::{im_str, Ui};
pub use inspect::Inspect;
pub use position::*;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use strum::IntoEnumIterator;
use strum::{Display, EnumIter};

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

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ConstructDefault<T> {
    _marker: PhantomData<T>,
}

impl<T: hecs::Component + Default> Construct for ConstructDefault<T> {
    type Context = ();
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        _: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(T::default()))
    }
}

impl<T> Inspect for ConstructDefault<T> {}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ConstructId<Id> {
    pub value: Id,
}

impl<Id: hecs::Component + Clone> Construct for ConstructId<Id> {
    type Context = ();
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        _: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        Ok(builder.add(self.value.clone()))
    }
}

impl<Id: IntoEnumIterator + Clone + Display + Eq> Inspect for ConstructId<Id> {
    fn inspect_mut(&mut self, ui: &Ui<'_>) {
        ui.combo_items(
            im_str!("Value"),
            &mut self.value,
            &Id::iter().collect::<Vec<_>>(),
            &|i| im_str!("{}", i).into(),
        );
    }
}

impl<Tag: ConstructTag> Inspect for Tag {
    fn inspect_mut(&mut self, _: &Ui<'_>) {}
}

impl<Context, T, U> Construct for (T, U)
where
    Context: Copy,
    T: Construct<Context = Context>,
    U: Construct<Context = Context>,
{
    type Context = Context;
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        self.0.construct_on_to(builder, context)?;
        self.1.construct_on_to(builder, context)?;
        Ok(builder)
    }
}

impl<Context, T, U, V> Construct for (T, U, V)
where
    Context: Copy,
    T: Construct<Context = Context>,
    U: Construct<Context = Context>,
    V: Construct<Context = Context>,
{
    type Context = Context;
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        self.0.construct_on_to(builder, context)?;
        self.1.construct_on_to(builder, context)?;
        self.2.construct_on_to(builder, context)?;
        Ok(builder)
    }
}

impl<T, U> Inspect for (T, U)
where
    T: Inspect,
    U: Inspect,
{
    fn inspect_mut(&mut self, ui: &Ui<'_>) {
        self.0.inspect_mut(ui);
        self.1.inspect_mut(ui);
    }
}
impl<T, U, V> Inspect for (T, U, V)
where
    T: Inspect,
    U: Inspect,
    V: Inspect,
{
    fn inspect_mut(&mut self, ui: &Ui<'_>) {
        self.0.inspect_mut(ui);
        self.1.inspect_mut(ui);
        self.2.inspect_mut(ui);
    }
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
impl TryAsMut<ContextlessConstructor> for Constructor {
    fn try_as_mut(&mut self) -> Option<&mut ContextlessConstructor> {
        if let Constructor::Contextless(ref mut value) = self {
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

impl TryAsMut<PositionConstructor> for Constructor {
    fn try_as_mut(&mut self) -> Option<&mut PositionConstructor> {
        if let Constructor::Position(ref mut value) = self {
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
pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

macro_rules! construct_enum_impl {
    (Construct<Context = $context:ty> for enum $name:ident { $($variant_name:ident($variant_type:ty),)+ }) => {
        #[enum_dispatch(Inspect)]
        #[derive(Serialize, Deserialize, Clone, EnumIter, Display, Eq, PartialEq)]
        pub enum $name {
            $($variant_name($variant_type)),+
        }

        $(
            impl Into<Constructor> for $variant_type {
                fn into(self) -> Constructor {
                    $name::from(self).into()
                }
            }
            impl TryInto<$variant_type> for Constructor {
                type Error = &'static str;
                fn try_into(self) -> Result<$variant_type, Self::Error> {
                    let value: $name = self.try_into()?;
                    value.try_into()
                }
            }
            #[allow(irrefutable_let_patterns)]
            impl TryAsRef<$variant_type> for Constructor {
                 fn try_as_ref(&self) -> Option<&$variant_type> {
                    let value: &$name = self.try_as_ref()?;
                    if let $name::$variant_name(ref value) = value {
                        Some(value)
                    } else {
                        None
                    }
                }
            }
            #[allow(irrefutable_let_patterns)]
            impl TryAsMut<$variant_type> for Constructor {
                 fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    let value: &mut $name = self.try_as_mut()?;
                    if let $name::$variant_name(ref mut value) = value {
                        Some(value)
                    } else {
                        None
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
                        Self::$variant_name(ref value) => value.construct_on_to(builder, context),
                    )+
                }
            }
        }

    };
}

construct_enum_impl!(
    Construct<Context = ()> for
    enum ContextlessConstructor {
        GlobalParticle((ConstructId<GlobalGraphic>, ExpiresAfterAnimation, ConstructDefault<Timer>)),
        YuyukoParticle((ConstructId<YuyukoGraphic>, ExpiresAfterAnimation, ConstructDefault<Timer>)),
    }
);
construct_enum_impl!(
    Construct<Context = collision::Vec2> for
    enum PositionConstructor {
        Position(Position),
    }
);
