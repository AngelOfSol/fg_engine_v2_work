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
pub use inspect::InspectOld;
use inspect_design::Inspect;
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

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Inspect)]
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

impl<T> InspectOld for ConstructDefault<T> {}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Inspect)]
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

impl<Id: IntoEnumIterator + Clone + Display + Eq> InspectOld for ConstructId<Id> {
    fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
        ui.combo_items(
            im_str!("Value"),
            &mut self.value,
            &Id::iter().collect::<Vec<_>>(),
            &|i| im_str!("{}", i).into(),
        );
    }
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
impl<Context, T, U, V, W> Construct for (T, U, V, W)
where
    Context: Copy,
    T: Construct<Context = Context>,
    U: Construct<Context = Context>,
    V: Construct<Context = Context>,
    W: Construct<Context = Context>,
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
        self.3.construct_on_to(builder, context)?;
        Ok(builder)
    }
}

impl<T, U> InspectOld for (T, U)
where
    T: InspectOld,
    U: InspectOld,
{
    fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
        self.0.inspect_mut_old(ui);
        self.1.inspect_mut_old(ui);
    }
}
impl<T, U, V> InspectOld for (T, U, V)
where
    T: InspectOld,
    U: InspectOld,
    V: InspectOld,
{
    fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
        self.0.inspect_mut_old(ui);
        self.1.inspect_mut_old(ui);
        self.2.inspect_mut_old(ui);
    }
}
impl<T, U, V, W> InspectOld for (T, U, V, W)
where
    T: InspectOld,
    U: InspectOld,
    V: InspectOld,
    W: InspectOld,
{
    fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
        self.0.inspect_mut_old(ui);
        self.1.inspect_mut_old(ui);
        self.2.inspect_mut_old(ui);
        self.3.inspect_mut_old(ui);
    }
}

impl InspectOld for () {}
impl Construct for () {
    type Context = ();
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        b: &'builder mut EntityBuilder,
        _: Self::Context,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        //
        Ok(b)
    }
}

#[enum_dispatch]
trait Auto {}

/// Each variant represents what context needs to be provided to the constructor.
#[enum_dispatch(Auto, InspectOld)]
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Inspect)]
#[no_label]
pub enum Constructor {
    Contextless(ContextlessConstructor),
    Position(PositionConstructor),
}
impl Default for Constructor {
    fn default() -> Self {
        Self::Contextless(Default::default())
    }
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
        #[enum_dispatch(InspectOld)]
        #[derive(Serialize, Deserialize, Clone, EnumIter, Display, Eq, PartialEq, Inspect)]
        #[no_label]
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

type ParticleData<T> = (
    ConstructId<T>,
    ConstructDefault<ExpiresAfterAnimation>,
    ConstructDefault<Timer>,
);

construct_enum_impl!(
    Construct<Context = ()> for
    enum ContextlessConstructor {
        GlobalParticle(ParticleData<GlobalGraphic>),
        YuyukoParticle(ParticleData<YuyukoGraphic>),

    }
);

impl Default for ContextlessConstructor {
    fn default() -> Self {
        Self::GlobalParticle(Default::default())
    }
}

construct_enum_impl!(
    Construct<Context = collision::Vec2> for
    enum PositionConstructor {
        Position(Position),
    }
);

impl Default for PositionConstructor {
    fn default() -> Self {
        Self::Position(Default::default())
    }
}
