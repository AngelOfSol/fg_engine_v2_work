use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
};

use crate::game_object::constructors::{TryAsMut, TryAsRef};

macro_rules! impl_property_type {
    (
        pub enum PropertyType {
            $($variant_name:ident($variant_type:ty),)+
        }
    ) => {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub enum PropertyType {
            $($variant_name($variant_type),)+
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
        )+
    };
}

impl_property_type! {
    pub enum PropertyType {
        Int(i32),
        Float(f32),
    }
}

pub struct InstanceData<DataId> {
    data: HashMap<(TypeId, DataId), PropertyType>,
}

impl<DataId> Serialize for InstanceData<DataId>
where
    DataId: Serialize + Hash + Eq,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data
            .iter()
            .map(|((type_id, key), value)| {
                (
                    (
                        inventory::iter::<Mapping>
                            .into_iter()
                            .find(|item| item.type_id == *type_id)
                            .map(|item| &item.name)
                            .unwrap(),
                        key,
                    ),
                    value,
                )
            })
            .collect::<HashMap<(&String, &DataId), &PropertyType>>()
            .serialize(serializer)
    }
}

impl<'de, DataId> Deserialize<'de> for InstanceData<DataId>
where
    DataId: Deserialize<'de> + Hash + Eq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: HashMap<(String, DataId), PropertyType> = HashMap::deserialize(deserializer)?;
        Ok(Self {
            data: data
                .into_iter()
                .map(|((type_name, key), value)| {
                    (
                        (
                            inventory::iter::<Mapping>
                                .into_iter()
                                .find(|item| item.name == type_name)
                                .map(|item| item.type_id)
                                .unwrap(),
                            key,
                        ),
                        value,
                    )
                })
                .collect(),
        })
    }
}

impl<DataId: Hash + Eq> InstanceData<DataId> {
    pub fn get<T>(&self, key: DataId) -> Option<&T>
    where
        T: Any,
        PropertyType: TryAsRef<T>,
    {
        self.data
            .get(&(TypeId::of::<T>(), key))
            .and_then(|item| item.try_as_ref())
    }
    pub fn get_mut<T>(&mut self, key: DataId) -> Option<&mut T>
    where
        T: Any,
        PropertyType: TryAsMut<T>,
    {
        self.data
            .get_mut(&(TypeId::of::<T>(), key))
            .and_then(|item| item.try_as_mut())
    }

    pub fn insert<T>(&mut self, key: DataId, value: T) -> Option<PropertyType>
    where
        T: Any,
        PropertyType: From<T>,
    {
        self.data
            .insert((TypeId::of::<T>(), key), PropertyType::from(value))
    }
    pub fn remove<T>(&mut self, key: DataId) -> Option<PropertyType>
    where
        T: Any,
        PropertyType: From<T>,
    {
        self.data.remove(&(TypeId::of::<T>(), key))
    }
}

inventory::collect!(Mapping);

struct Mapping {
    name: String,
    type_id: TypeId,
}

impl Mapping {
    fn new<T: Any>(name: String) -> Self {
        Self {
            name,
            type_id: TypeId::of::<T>(),
        }
    }
}
