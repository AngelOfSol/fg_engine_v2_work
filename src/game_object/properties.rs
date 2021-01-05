#[macro_use]
mod macros;

pub mod typedefs;

use crate::character::state::components::GlobalGraphic;
use inspect_design::Inspect;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use typedefs::Speed;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    convert::TryInto,
    hash::Hash,
};
use strum::{Display, EnumIter};

pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}
pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

impl_property_type! {
    pub enum PropertyType {
        GlobalGraphic(GlobalGraphic),
        YuyukoGraphic(crate::roster::yuyuko::Graphic),
        Speed(Speed),
    }
}

impl PropertyType {
    pub fn same_type_as(&self, rhs: &Self) -> bool {
        self.inner_type_id() == rhs.inner_type_id()
    }
}

impl Default for PropertyType {
    fn default() -> Self {
        PropertyType::GlobalGraphic(Default::default())
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

impl<DataId: Hash + Eq> Default for InstanceData<DataId> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
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

    pub fn insert<T>(&mut self, key: DataId, value: T) -> Option<T>
    where
        T: Any,
        PropertyType: From<T> + TryInto<T>,
    {
        self.data
            .insert((TypeId::of::<T>(), key), PropertyType::from(value))
            .and_then(|i| i.try_into().ok())
    }
    pub fn insert_any(&mut self, key: DataId, value: PropertyType) -> Option<PropertyType> {
        self.data.insert((value.inner_type_id(), key), value)
    }

    pub fn remove<T>(&mut self, key: DataId) -> Option<T>
    where
        T: Any,
        PropertyType: From<T> + TryInto<T>,
    {
        self.data
            .remove(&(TypeId::of::<T>(), key))
            .and_then(|i| i.try_into().ok())
    }

    pub fn remove_any(&mut self, key: DataId, type_id: TypeId) -> Option<PropertyType> {
        self.data.remove(&(type_id, key))
    }
}

impl<DataId: Hash + Eq + Clone> InstanceData<DataId> {
    pub fn iter_key(&self, key: DataId) -> impl Iterator<Item = (String, &PropertyType)> {
        self.data
            .iter()
            .filter(move |((_, data_id), _)| &key == data_id)
            .map(|((type_id, _), value)| {
                (
                    inventory::iter::<Mapping>
                        .into_iter()
                        .find(|item| &item.type_id == type_id)
                        .map(|item| item.name.clone())
                        .unwrap(),
                    value,
                )
            })
    }
    pub fn iter_key_mut(
        &mut self,
        key: DataId,
    ) -> impl Iterator<Item = (String, &mut PropertyType)> {
        self.data
            .iter_mut()
            .filter(move |((_, data_id), _)| &key == data_id)
            .map(|((type_id, _), value)| {
                (
                    inventory::iter::<Mapping>
                        .into_iter()
                        .find(|item| &item.type_id == type_id)
                        .map(|item| item.name.clone())
                        .unwrap(),
                    value,
                )
            })
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
    use crate::roster::yuyuko::Graphic;

    let mut props = InstanceData::new();
    props.insert(0, Graphic::HitEffect);
    props.insert(1, GlobalGraphic::SuperJump);
    props.insert_any(2, Graphic::SuperJumpParticle.into());
    props.insert(2, GlobalGraphic::SuperJump);

    let string_variant = serde_json::to_string(&props);
    let round_trip: InstanceData<i32> = serde_json::from_str(&string_variant.unwrap()).unwrap();
    assert_eq!(props, round_trip);

    assert_eq!(round_trip.get::<Graphic>(0), Some(&Graphic::HitEffect));
    assert_eq!(round_trip.get(1), None::<&Graphic>);
    assert_eq!(
        round_trip.get::<Graphic>(2),
        Some(&Graphic::SuperJumpParticle)
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

    assert_eq!(round_trip.exists::<Graphic>(0), true);
    assert_eq!(round_trip.exists::<Graphic>(1), false);
    assert_eq!(round_trip.exists::<Graphic>(2), true);

    assert_eq!(round_trip.exists::<GlobalGraphic>(0), false);
    assert_eq!(round_trip.exists::<GlobalGraphic>(1), true);
    assert_eq!(round_trip.exists::<GlobalGraphic>(2), true);

    let mut round_trip = round_trip;

    round_trip.remove::<GlobalGraphic>(1);
    round_trip.remove_any(
        2,
        PropertyType::from(Graphic::SuperJumpParticle).inner_type_id(),
    );
    assert_eq!(round_trip.exists::<Graphic>(0), true);
    assert_eq!(round_trip.exists::<Graphic>(1), false);
    assert_eq!(round_trip.exists::<Graphic>(2), false);

    assert_eq!(round_trip.exists::<GlobalGraphic>(0), false);
    assert_eq!(round_trip.exists::<GlobalGraphic>(1), false);
    assert_eq!(round_trip.exists::<GlobalGraphic>(2), true);
}
