macro_rules! impl_property_type {
    (
        pub enum PropertyType {
            $($variant_name:ident($variant_type:ty),)+
        }
    ) => {
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumIter, Display, Inspect)]
        #[static_enum]
        pub enum PropertyType {
            $($variant_name($variant_type),)+
        }


        impl PropertyType {
            pub fn inner_type_id(&self) -> TypeId {
                match self {
                    $(
                        PropertyType::$variant_name(_) => TypeId::of::<$variant_type>(),
                    )+
                }
            }
        }

        $(
            inventory::submit!(Mapping::new::<$variant_type>(stringify!($variant_name).to_owned()));
            impl From<$variant_type> for PropertyType {
                fn from(value: $variant_type) -> Self {
                    PropertyType::$variant_name(value)
                }
            }
            impl TryAsRef<$variant_type> for PropertyType {
                fn try_as_ref(&self) -> Option<&$variant_type> {
                    if let PropertyType::$variant_name(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            }
            impl TryAsMut<$variant_type> for PropertyType {
                fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    if let PropertyType::$variant_name(value) = self {
                        Some(value)
                    } else {
                        None
                    }
                }
            }
            impl TryInto<$variant_type> for PropertyType {
                type Error = &'static str;
                fn try_into(self) -> Result<$variant_type, Self::Error> {
                    if let PropertyType::$variant_name(value) = self {
                        Ok(value)
                    } else {
                        Err(concat!("value isn't type ", stringify!($variant_type)))
                    }
                }
            }
        )+
    };
}
