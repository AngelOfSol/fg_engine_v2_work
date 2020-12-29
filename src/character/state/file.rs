use super::State;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::PathBuf;

pub fn load_from_json<
    Id: DeserializeOwned + Serialize + Eq + Hash + Default,
    AttackId: DeserializeOwned + Serialize + Default,
    SoundType: DeserializeOwned + Serialize + Default,
>(
    path: PathBuf,
) -> State<Id, AttackId, SoundType> {
    let file = File::open(&path).unwrap();
    let buf_read = BufReader::new(file);
    serde_json::from_reader::<_, State<_, _, _>>(buf_read).unwrap()
}
pub fn save<Id: Serialize + Eq + Hash, AttackId: Serialize, SoundType: Serialize>(
    state: &State<Id, AttackId, SoundType>,
    path: PathBuf,
) {
    let mut json = File::create(&path).unwrap();
    serde_json::to_writer(&mut json, &state).unwrap();
}
