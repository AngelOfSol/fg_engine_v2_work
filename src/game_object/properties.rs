use crate::{
    character::state::components::GlobalGraphic,
    game_object::constructors::{TryAsMut, TryAsRef},
    roster::YuyukoGraphic,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
};
use strum::EnumIter;

macro_rules! impl_property_type {
    (
        pub enum PropertyType {
            $($variant_name:ident($variant_type:ty),)+
        }
    ) => {
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumIter)]
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
        GlobalGraphic(GlobalGraphic),
        YuyukoGraphic(YuyukoGraphic),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InstanceData<DataId>
where
    HashMap<(TypeId, DataId), PropertyType>: PartialEq + Eq,
{
    data: HashMap<(TypeId, DataId), PropertyType>,
}

impl<DataId> Serialize for InstanceData<DataId>
where
    DataId: Serialize + Hash + Eq + std::fmt::Debug,
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
            .collect::<Vec<((&String, &DataId), &PropertyType)>>()
            .serialize(serializer)
    }
}

impl<'de, DataId> Deserialize<'de> for InstanceData<DataId>
where
    DataId: Deserialize<'de> + Hash + Eq,
    (String, DataId): Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: Vec<((String, DataId), PropertyType)> = Vec::deserialize(deserializer)?;
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
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
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
    pub fn exists<T>(&self, key: DataId) -> bool
    where
        T: Any,
    {
        self.data.contains_key(&(TypeId::of::<T>(), key))
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
    fn new<T: Default + Any>(name: String) -> Self
    where
        PropertyType: From<T>,
    {
        Self {
            name,
            type_id: TypeId::of::<T>(),
        }
    }
}

#[test]
fn test_instance_data() {
    let mut props = InstanceData::new();
    props.insert(0, YuyukoGraphic::HitEffect);
    props.insert(1, GlobalGraphic::SuperJump);
    props.insert(2, YuyukoGraphic::SuperJumpParticle);
    props.insert(2, GlobalGraphic::SuperJump);

    let string_variant = serde_json::to_string(&props);
    let round_trip: InstanceData<i32> = serde_json::from_str(&string_variant.unwrap()).unwrap();
    assert_eq!(props, round_trip);

    assert_eq!(
        round_trip.get::<YuyukoGraphic>(0),
        Some(&YuyukoGraphic::HitEffect)
    );
    assert_eq!(round_trip.get(1), None::<&YuyukoGraphic>);
    assert_eq!(
        round_trip.get::<YuyukoGraphic>(2),
        Some(&YuyukoGraphic::SuperJumpParticle)
    );

    assert_eq!(round_trip.get(0), None::<&GlobalGraphic>);
    assert_eq!(
        round_trip.get::<GlobalGraphic>(1),
        Some(&GlobalGraphic::SuperJump)
    );
    assert_eq!(
        round_trip.get::<GlobalGraphic>(2),
        Some(&GlobalGraphic::SuperJump)
    );

    assert_eq!(round_trip.exists::<YuyukoGraphic>(0), true);
    assert_eq!(round_trip.exists::<YuyukoGraphic>(1), false);
    assert_eq!(round_trip.exists::<YuyukoGraphic>(2), true);

    assert_eq!(round_trip.exists::<GlobalGraphic>(0), false);
    assert_eq!(round_trip.exists::<GlobalGraphic>(1), true);
    assert_eq!(round_trip.exists::<GlobalGraphic>(2), true);
}
