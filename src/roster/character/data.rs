use super::{player_state::PlayerState, typedefs::Character};
use crate::{
    assets::Assets,
    character::{
        command::Command,
        components::{AttackInfo, Properties},
        state::{State, StateInstant},
    },
    game_match::sounds::SoundList,
    game_object::properties::InstanceData,
    graphics::animation_group::AnimationGroup,
};
use fg_input::Input;
use ggez::{Context, GameError, GameResult};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::PathBuf,
};
use strum::IntoEnumIterator;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Data<C: Character> {
    pub states: HashMap<C::State, State<C>>,
    pub attacks: HashMap<C::Attack, AttackInfo>,
    pub properties: Properties,
    #[serde(skip)]
    pub sounds: SoundList<C::Sound>,
    pub graphics: HashMap<C::Graphic, AnimationGroup>,
    pub input_map: BTreeMap<Input, Vec<C::Command>>,
    pub command_map: HashMap<C::Command, Command<C>>,
    pub state_graphics_map: HashMap<C::State, C::Graphic>,
    pub instance: InstanceData<C::ObjectData>,
    #[serde(default)]
    pub other: C::StaticData,
}

impl<C: Character> Data<C> {
    pub fn get<'this, 'state>(
        &'this self,
        state: &'state PlayerState<C>,
    ) -> StateInstant<'this, C> {
        self.states[&state.current_state.id].get(state.current_state.time)
    }
    pub fn get_next<'this, 'state>(
        &'this self,
        state: &'state PlayerState<C>,
    ) -> StateInstant<'this, C> {
        self.states[&state.current_state.id].get(state.current_state.time + 1)
    }
}
impl<C: Character> Data<C>
where
    Data<C>: DeserializeOwned + Serialize,
{
    pub fn new_with_path(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<Self> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let mut character = serde_json::from_reader::<_, Self>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        path.push(&name);

        path.push("sounds");
        for sound in C::Sound::iter() {
            path.push(format!("{}.mp3", sound));
            use rodio::source::Source;
            let source =
                rodio::decoder::Decoder::new(std::io::BufReader::new(std::fs::File::open(&path)?))
                    .unwrap();
            let source = rodio::buffer::SamplesBuffer::new(
                source.channels(),
                source.sample_rate(),
                source.convert_samples().collect::<Vec<_>>(),
            )
            .buffered();

            character.sounds.data.insert(sound, source);
            path.pop();
        }

        path.pop();
        path.push("graphics");
        for (name, animation_group) in character.graphics.iter_mut() {
            path.push(format!("{}", name));
            AnimationGroup::load(ctx, assets, animation_group, path.clone())?;
            path.pop();
        }
        Ok(character)
    }

    pub fn save(
        ctx: &mut Context,
        assets: &mut Assets,
        player_character: &Self,
        mut path: PathBuf,
    ) -> GameResult<()> {
        let character_file_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        let mut json = File::create(&path)?;
        serde_json::to_writer(&mut json, &player_character)
            .map_err(|err| GameError::FilesystemError(format!("{}", err)))?;

        path.pop();
        path.push(&character_file_name);

        path.push("graphics");
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir_all(&path)?;
        for (name, animation_group) in player_character.graphics.iter() {
            path.push(format!("{}.json", name));
            AnimationGroup::save(ctx, assets, animation_group, path.clone())?;
            path.pop();
        }
        path.pop();
        Ok(())
    }
}
