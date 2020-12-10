pub mod constructors {
    mod position {
        use super::{ConstructError, PositionConstruct};
        use crate::{game_object::state::Position, typedefs::collision};
        use hecs::EntityBuilder;
        impl PositionConstruct for Position {
            fn construct_on_to<'constructor>(
                &'constructor self,
                builder: &mut EntityBuilder,
                offset: collision::Vec2,
            ) -> Result<&mut EntityBuilder, ConstructError> {
                Ok(builder.add(Self {
                    value: self.value + offset,
                }))
            }
        }
    }
    pub use position::*;
    use super::state::{Position, Render};
    use crate::typedefs::collision;
    use enum_dispatch::*;
    use hecs::EntityBuilder;
    use serde::{Deserialize, Serialize};
    pub enum ConstructError {
        MissingRequiredComponent,
    }
    pub trait ContextlessConstruct {
        fn construct_on_to(
            &self,
            builder: &mut EntityBuilder,
            context: (),
        ) -> Result<&mut EntityBuilder, ConstructError>;
    }
    pub trait PositionConstruct {
        fn construct_on_to(
            &self,
            builder: &mut EntityBuilder,
            context: collision::Vec2,
        ) -> Result<&mut EntityBuilder, ConstructError>;
    }
    pub trait ConstructTag: Default {}
    /// Each variant represents what context needs to be provided to the constructor.
    pub enum Constructor {
        Contextless(ContextlessConstructor),
        Position(PositionConstructor),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Constructor {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Constructor::Contextless(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Constructor",
                            0u32,
                            "Contextless",
                            __field0,
                        )
                    }
                    Constructor::Position(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Constructor",
                            1u32,
                            "Position",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Constructor {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            1u64 => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Contextless" => _serde::export::Ok(__Field::__field0),
                            "Position" => _serde::export::Ok(__Field::__field1),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Contextless" => _serde::export::Ok(__Field::__field0),
                            b"Position" => _serde::export::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<Constructor>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Constructor;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "enum Constructor")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => _serde::export::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<ContextlessConstructor>(
                                    __variant,
                                ),
                                Constructor::Contextless,
                            ),
                            (__Field::__field1, __variant) => _serde::export::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<PositionConstructor>(
                                    __variant,
                                ),
                                Constructor::Position,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Contextless", "Position"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Constructor",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<Constructor>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Constructor {
        #[inline]
        fn clone(&self) -> Constructor {
            match (&*self,) {
                (&Constructor::Contextless(ref __self_0),) => {
                    Constructor::Contextless(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Constructor::Position(ref __self_0),) => {
                    Constructor::Position(::core::clone::Clone::clone(&(*__self_0)))
                }
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
        fn iter() -> impl Iterator<Item = Constructor> {
            let test = Render::default();
            let test: ContextlessConstructor = test.into();
            None.into_iter()
        }
    }
    pub enum ContextlessConstructor {
        Render(Render),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ContextlessConstructor {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    ContextlessConstructor::Render(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "ContextlessConstructor",
                            0u32,
                            "Render",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ContextlessConstructor {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Render" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Render" => _serde::export::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<ContextlessConstructor>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ContextlessConstructor;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "enum ContextlessConstructor",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => _serde::export::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Render>(__variant),
                                ContextlessConstructor::Render,
                            ),
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Render"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ContextlessConstructor",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<ContextlessConstructor>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for ContextlessConstructor {
        #[inline]
        fn clone(&self) -> ContextlessConstructor {
            match (&*self,) {
                (&ContextlessConstructor::Render(ref __self_0),) => {
                    ContextlessConstructor::Render(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    impl ::core::convert::From<Render> for ContextlessConstructor {
        fn from(v: Render) -> ContextlessConstructor {
            ContextlessConstructor::Render(v)
        }
    }
    impl core::convert::TryInto<Render> for ContextlessConstructor {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<Render, <Self as core::convert::TryInto<Render>>::Error>
        {
            match self {
                ContextlessConstructor::Render(v) => Ok(v),
            }
        }
    }
    impl ContextlessConstruct for ContextlessConstructor {
        #[inline]
        fn construct_on_to(
            &self,
            builder: &mut EntityBuilder,
            context: (),
        ) -> Result<&mut EntityBuilder, ConstructError> {
            match self {
                ContextlessConstructor::Render(inner) => {
                    ContextlessConstruct::construct_on_to(inner, builder, context)
                }
            }
        }
    }
    pub enum PositionConstructor {
        Position,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PositionConstructor {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    PositionConstructor::Position => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "PositionConstructor",
                        0u32,
                        "Position",
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PositionConstructor {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Position" => _serde::export::Ok(__Field::__field0),
                            _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::export::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Position" => _serde::export::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::export::from_utf8_lossy(__value);
                                _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<PositionConstructor>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PositionConstructor;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "enum PositionConstructor",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match match _serde::de::EnumAccess::variant(__data) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            (__Field::__field0, __variant) => {
                                match _serde::de::VariantAccess::unit_variant(__variant) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::export::Ok(PositionConstructor::Position)
                            }
                        }
                    }
                }
                const VARIANTS: &'static [&'static str] = &["Position"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "PositionConstructor",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::export::PhantomData::<PositionConstructor>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for PositionConstructor {
        #[inline]
        fn clone(&self) -> PositionConstructor {
            match (&*self,) {
                (&PositionConstructor::Position,) => PositionConstructor::Position,
            }
        }
    }
}
