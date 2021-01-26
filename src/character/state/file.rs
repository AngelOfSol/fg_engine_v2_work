use crate::roster::character::{data::Data, typedefs::Character};

use super::State;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_from_json<C: Character>(path: PathBuf) -> State<C>
where
    Data<C>: Serialize + for<'de> Deserialize<'de>,
{
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    serde_json::from_reader::<_, State<_>>(buf_read).unwrap()
}
pub fn save<C: Character>(state: &State<C>, path: PathBuf)
where
    Data<C>: Serialize + for<'de> Deserialize<'de>,
{
    let mut json = File::create(&path).unwrap();
    serde_json::to_writer(&mut json, &state).unwrap();
}
