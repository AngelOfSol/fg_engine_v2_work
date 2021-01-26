mod hitbox_set {
    use crate::{hitbox::Hitbox, roster::character::typedefs::Character};
    use inspect_design::Inspect;
    use serde::{Deserialize, Serialize};
    pub struct AttackData<C> {
        pub id: usize,
        pub boxes: Vec<Hitbox>,
        pub data_id: C,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::fmt::Debug> ::core::fmt::Debug for AttackData<C> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                AttackData {
                    id: ref __self_0_0,
                    boxes: ref __self_0_1,
                    data_id: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("AttackData");
                    let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("boxes", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("data_id", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::clone::Clone> ::core::clone::Clone for AttackData<C> {
        #[inline]
        fn clone(&self) -> AttackData<C> {
            match *self {
                AttackData {
                    id: ref __self_0_0,
                    boxes: ref __self_0_1,
                    data_id: ref __self_0_2,
                } => AttackData {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    boxes: ::core::clone::Clone::clone(&(*__self_0_1)),
                    data_id: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, C> _serde::Deserialize<'de> for AttackData<C>
        where
            C: _serde::Deserialize<'de>,
        {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "boxes" => _serde::__private::Ok(__Field::__field1),
                            "data_id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"boxes" => _serde::__private::Ok(__Field::__field1),
                            b"data_id" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, C>
                where
                    C: _serde::Deserialize<'de>,
                {
                    marker: _serde::__private::PhantomData<AttackData<C>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, C> _serde::de::Visitor<'de> for __Visitor<'de, C>
                where
                    C: _serde::Deserialize<'de>,
                {
                    type Value = AttackData<C>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct AttackData")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AttackData with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Vec<Hitbox>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct AttackData with 3 elements",
                                ));
                            }
                        };
                        let __field2 =
                            match match _serde::de::SeqAccess::next_element::<C>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct AttackData with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(AttackData {
                            id: __field0,
                            boxes: __field1,
                            data_id: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<usize> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<Hitbox>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<C> = _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<usize>(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "boxes",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Hitbox>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data_id",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<C>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("boxes") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("data_id") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(AttackData {
                            id: __field0,
                            boxes: __field1,
                            data_id: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["id", "boxes", "data_id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "AttackData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AttackData<C>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<C> ::core::marker::StructuralPartialEq for AttackData<C> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::cmp::PartialEq> ::core::cmp::PartialEq for AttackData<C> {
        #[inline]
        fn eq(&self, other: &AttackData<C>) -> bool {
            match *other {
                AttackData {
                    id: ref __self_1_0,
                    boxes: ref __self_1_1,
                    data_id: ref __self_1_2,
                } => match *self {
                    AttackData {
                        id: ref __self_0_0,
                        boxes: ref __self_0_1,
                        data_id: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &AttackData<C>) -> bool {
            match *other {
                AttackData {
                    id: ref __self_1_0,
                    boxes: ref __self_1_1,
                    data_id: ref __self_1_2,
                } => match *self {
                    AttackData {
                        id: ref __self_0_0,
                        boxes: ref __self_0_1,
                        data_id: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<C> _serde::Serialize for AttackData<C>
        where
            C: _serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "AttackData",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "boxes",
                    &self.boxes,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "data_id",
                    &self.data_id,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    pub struct AttackDataUiState<C>
    where
        usize: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        C: inspect_design::traits::Inspect,
    {
        id: <usize as inspect_design::traits::Inspect>::State,
        boxes: <Vec<Hitbox> as inspect_design::traits::Inspect>::State,
        data_id: <C as inspect_design::traits::Inspect>::State,
        phantom_c: std::marker::PhantomData<C>,
    }
    impl<C> Default for AttackDataUiState<C>
    where
        usize: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        C: inspect_design::traits::Inspect,
    {
        fn default() -> Self {
            Self {
                id: <<usize as inspect_design::traits::Inspect>::State as Default>::default(),
                boxes:
                    <<Vec<Hitbox> as inspect_design::traits::Inspect>::State as Default>::default(),
                data_id: <<C as inspect_design::traits::Inspect>::State as Default>::default(),
                phantom_c: std::marker::PhantomData,
            }
        }
    }
    impl<C> inspect_design::traits::Inspect for AttackData<C>
    where
        usize: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        C: inspect_design::traits::Inspect,
    {
        type State = AttackDataUiState<C>;
        fn inspect(&self, label: &str, state: &mut Self::State, ui: &imgui::Ui<'_>) {
            let id = ui.push_id(label);
            ui.group(|| {
                if <usize as inspect_design::traits::Inspect>::FLATTEN {
                    self.id.inspect("id", &mut state.id, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"id",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.id.inspect("id", &mut state.id, ui);
                        ui.unindent();
                    });
                }
                if <Vec<Hitbox> as inspect_design::traits::Inspect>::FLATTEN {
                    self.boxes.inspect("boxes", &mut state.boxes, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"boxes",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.boxes.inspect("boxes", &mut state.boxes, ui);
                        ui.unindent();
                    });
                }
                if <C as inspect_design::traits::Inspect>::FLATTEN {
                    self.data_id.inspect("data_id", &mut state.data_id, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"data_id",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.data_id.inspect("data_id", &mut state.data_id, ui);
                        ui.unindent();
                    });
                }
            });
            id.pop(ui);
        }
    }
    impl<C> inspect_design::traits::InspectMut for AttackData<C>
    where
        usize: inspect_design::traits::InspectMut,
        Vec<Hitbox>: inspect_design::traits::InspectMut,
        C: inspect_design::traits::InspectMut,
    {
        fn inspect_mut(&mut self, label: &str, state: &mut Self::State, ui: &imgui::Ui<'_>) {
            let id = ui.push_id(label);
            ui.group(|| {
                if <usize as inspect_design::traits::Inspect>::FLATTEN {
                    self.id.inspect_mut("id", &mut state.id, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"id",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.id.inspect_mut("id", &mut state.id, ui);
                        ui.unindent();
                    });
                }
                if <Vec<Hitbox> as inspect_design::traits::Inspect>::FLATTEN {
                    self.boxes.inspect_mut("boxes", &mut state.boxes, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"boxes",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.boxes.inspect_mut("boxes", &mut state.boxes, ui);
                        ui.unindent();
                    });
                }
                if <C as inspect_design::traits::Inspect>::FLATTEN {
                    self.data_id.inspect_mut("data_id", &mut state.data_id, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"data_id",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.data_id.inspect_mut("data_id", &mut state.data_id, ui);
                        ui.unindent();
                    });
                }
            });
            id.pop(ui);
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::default::Default> ::core::default::Default for AttackData<C> {
        #[inline]
        fn default() -> AttackData<C> {
            AttackData {
                id: ::core::default::Default::default(),
                boxes: ::core::default::Default::default(),
                data_id: ::core::default::Default::default(),
            }
        }
    }
    pub struct HitboxSet<C: Character> {
        pub collision: Hitbox,
        pub hurtbox: Vec<Hitbox>,
        pub hitbox: Option<AttackData<C::Attack>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::fmt::Debug + Character> ::core::fmt::Debug for HitboxSet<C>
    where
        C::Attack: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                HitboxSet {
                    collision: ref __self_0_0,
                    hurtbox: ref __self_0_1,
                    hitbox: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("HitboxSet");
                    let _ = debug_trait_builder.field("collision", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("hurtbox", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("hitbox", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::clone::Clone + Character> ::core::clone::Clone for HitboxSet<C>
    where
        C::Attack: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> HitboxSet<C> {
            match *self {
                HitboxSet {
                    collision: ref __self_0_0,
                    hurtbox: ref __self_0_1,
                    hitbox: ref __self_0_2,
                } => HitboxSet {
                    collision: ::core::clone::Clone::clone(&(*__self_0_0)),
                    hurtbox: ::core::clone::Clone::clone(&(*__self_0_1)),
                    hitbox: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<C: Character> _serde::Serialize for HitboxSet<C> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "HitboxSet",
                    false as usize + 1 + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "collision",
                    &self.collision,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "hurtbox",
                    &self.hurtbox,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "hitbox",
                    &self.hitbox,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, C: Character> _serde::Deserialize<'de> for HitboxSet<C> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "collision" => _serde::__private::Ok(__Field::__field0),
                            "hurtbox" => _serde::__private::Ok(__Field::__field1),
                            "hitbox" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"collision" => _serde::__private::Ok(__Field::__field0),
                            b"hurtbox" => _serde::__private::Ok(__Field::__field1),
                            b"hitbox" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de, C: Character> {
                    marker: _serde::__private::PhantomData<HitboxSet<C>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, C: Character> _serde::de::Visitor<'de> for __Visitor<'de, C> {
                    type Value = HitboxSet<C>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct HitboxSet")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match match _serde::de::SeqAccess::next_element::<Hitbox>(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct HitboxSet with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<Vec<Hitbox>>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct HitboxSet with 3 elements",
                                ));
                            }
                        };
                        let __field2 = match match _serde::de::SeqAccess::next_element::<
                            Option<AttackData<C::Attack>>,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct HitboxSet with 3 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(HitboxSet {
                            collision: __field0,
                            hurtbox: __field1,
                            hitbox: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Hitbox> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<Hitbox>> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<AttackData<C::Attack>>> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "collision",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Hitbox>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "hurtbox",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<Vec<Hitbox>>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "hitbox",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Option<AttackData<C::Attack>>,
                                        >(&mut __map)
                                        {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                _ => {
                                    let _ = match _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)
                                    {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    };
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("collision") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("hurtbox") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("hitbox") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(HitboxSet {
                            collision: __field0,
                            hurtbox: __field1,
                            hitbox: __field2,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["collision", "hurtbox", "hitbox"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "HitboxSet",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<HitboxSet<C>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct HitboxSetUiState<C: Character>
    where
        Hitbox: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        Option<AttackData<C::Attack>>: inspect_design::traits::Inspect,
    {
        collision: <Hitbox as inspect_design::traits::Inspect>::State,
        hurtbox: <Vec<Hitbox> as inspect_design::traits::Inspect>::State,
        hitbox: <Option<AttackData<C::Attack>> as inspect_design::traits::Inspect>::State,
        phantom_c: std::marker::PhantomData<C>,
    }
    impl<C: Character> Default for HitboxSetUiState<C>
    where
        Hitbox: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        Option<AttackData<C::Attack>>: inspect_design::traits::Inspect,
    {
        fn default() -> Self {
            Self { collision : < < Hitbox as inspect_design :: traits :: Inspect > :: State as Default > :: default () , hurtbox : < < Vec < Hitbox > as inspect_design :: traits :: Inspect > :: State as Default > :: default () , hitbox : < < Option < AttackData < C :: Attack > > as inspect_design :: traits :: Inspect > :: State as Default > :: default () , phantom_c : std :: marker :: PhantomData , }
        }
    }
    impl<C: Character> inspect_design::traits::Inspect for HitboxSet<C>
    where
        Hitbox: inspect_design::traits::Inspect,
        Vec<Hitbox>: inspect_design::traits::Inspect,
        Option<AttackData<C::Attack>>: inspect_design::traits::Inspect,
    {
        type State = HitboxSetUiState<C>;
        fn inspect(&self, label: &str, state: &mut Self::State, ui: &imgui::Ui<'_>) {
            let id = ui.push_id(label);
            ui.group(|| {
                if <Hitbox as inspect_design::traits::Inspect>::FLATTEN {
                    self.collision
                        .inspect("collision", &mut state.collision, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"collision",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.collision
                            .inspect("collision", &mut state.collision, ui);
                        ui.unindent();
                    });
                }
                if <Vec<Hitbox> as inspect_design::traits::Inspect>::FLATTEN {
                    self.hurtbox.inspect("hurtbox", &mut state.hurtbox, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"hurtbox",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.hurtbox.inspect("hurtbox", &mut state.hurtbox, ui);
                        ui.unindent();
                    });
                }
                if <Option<AttackData<C::Attack>> as inspect_design::traits::Inspect>::FLATTEN {
                    self.hitbox.inspect("hitbox", &mut state.hitbox, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"hitbox",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.hitbox.inspect("hitbox", &mut state.hitbox, ui);
                        ui.unindent();
                    });
                }
            });
            id.pop(ui);
        }
    }
    impl<C: Character> inspect_design::traits::InspectMut for HitboxSet<C>
    where
        Hitbox: inspect_design::traits::InspectMut,
        Vec<Hitbox>: inspect_design::traits::InspectMut,
        Option<AttackData<C::Attack>>: inspect_design::traits::InspectMut,
    {
        fn inspect_mut(&mut self, label: &str, state: &mut Self::State, ui: &imgui::Ui<'_>) {
            let id = ui.push_id(label);
            ui.group(|| {
                if <Hitbox as inspect_design::traits::Inspect>::FLATTEN {
                    self.collision
                        .inspect_mut("collision", &mut state.collision, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"collision",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.collision
                            .inspect_mut("collision", &mut state.collision, ui);
                        ui.unindent();
                    });
                }
                if <Vec<Hitbox> as inspect_design::traits::Inspect>::FLATTEN {
                    self.hurtbox.inspect_mut("hurtbox", &mut state.hurtbox, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"hurtbox",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.hurtbox.inspect_mut("hurtbox", &mut state.hurtbox, ui);
                        ui.unindent();
                    });
                }
                if <Option<AttackData<C::Attack>> as inspect_design::traits::Inspect>::FLATTEN {
                    self.hitbox.inspect_mut("hitbox", &mut state.hitbox, ui);
                } else {
                    ui.text(&{
                        unsafe {
                            ::imgui::ImString::from_utf8_with_nul_unchecked(
                                {
                                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                        &["", ":\u{0}"],
                                        &match (&"hitbox",) {
                                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                    res
                                }
                                .into_bytes(),
                            )
                        }
                    });
                    ui.group(|| {
                        ui.indent();
                        self.hitbox.inspect_mut("hitbox", &mut state.hitbox, ui);
                        ui.unindent();
                    });
                }
            });
            id.pop(ui);
        }
    }
    impl<C: Character> ::core::marker::StructuralPartialEq for HitboxSet<C> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::cmp::PartialEq + Character> ::core::cmp::PartialEq for HitboxSet<C>
    where
        C::Attack: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &HitboxSet<C>) -> bool {
            match *other {
                HitboxSet {
                    collision: ref __self_1_0,
                    hurtbox: ref __self_1_1,
                    hitbox: ref __self_1_2,
                } => match *self {
                    HitboxSet {
                        collision: ref __self_0_0,
                        hurtbox: ref __self_0_1,
                        hitbox: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &HitboxSet<C>) -> bool {
            match *other {
                HitboxSet {
                    collision: ref __self_1_0,
                    hurtbox: ref __self_1_1,
                    hitbox: ref __self_1_2,
                } => match *self {
                    HitboxSet {
                        collision: ref __self_0_0,
                        hurtbox: ref __self_0_1,
                        hitbox: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<C: ::core::default::Default + Character> ::core::default::Default for HitboxSet<C>
    where
        C::Attack: ::core::default::Default,
    {
        #[inline]
        fn default() -> HitboxSet<C> {
            HitboxSet {
                collision: ::core::default::Default::default(),
                hurtbox: ::core::default::Default::default(),
                hitbox: ::core::default::Default::default(),
            }
        }
    }
}
