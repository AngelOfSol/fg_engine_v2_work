use crate::game_match::{
    GlobalGraphic, GlobalSound, PlayArea, PlayerUi, RoundStartUi, ShieldUi, SoundList, Stage,
    UiElements,
};
use crate::graphics::animation_group::AnimationGroup;
use crate::player_list::PlayerList;
use crate::roster::CharacterData;
use crate::{assets::Assets, roster};
use fg_datastructures::{player_data::PlayerData, roster::RosterCharacter};
use ggez::{graphics, Context, GameResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use strum::IntoEnumIterator;

const REPLAY_VERSION: usize = 4;

#[derive(Clone, Serialize, Deserialize)]
pub struct MatchSettings {
    replay_version: usize,
    pub first_to: usize,
    pub characters: PlayerData<RosterCharacter>,
    #[serde(skip)]
    pub runtime_data: Option<Rc<RuntimeData>>,
}

#[derive(Clone)]
pub struct RuntimeData {
    pub character_data: PlayerData<CharacterData>,
    pub assets: Assets,
    pub sounds: SoundList<GlobalSound>,
    pub graphics: HashMap<GlobalGraphic, AnimationGroup>,
    pub ui: UiElements,
    pub background: Stage,
    pub play_area: PlayArea,
}

pub enum MatchSettingsError {
    ReplayVersionMismatch,
    DeserializeError(bincode::Error),
}

impl From<bincode::Error> for MatchSettingsError {
    fn from(value: bincode::Error) -> MatchSettingsError {
        MatchSettingsError::DeserializeError(value)
    }
}

impl MatchSettings {
    pub fn new() -> MatchSettings {
        MatchSettings {
            first_to: 2,
            characters: [RosterCharacter::default(); 2].into(),
            replay_version: REPLAY_VERSION,
            runtime_data: None,
        }
    }

    pub fn validate(&self) -> Result<(), MatchSettingsError> {
        if self.replay_version != REPLAY_VERSION {
            return Err(MatchSettingsError::ReplayVersionMismatch);
        }

        Ok(())
    }

    pub fn load(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.runtime_data {
            Some(ref mut data) => {
                let data = Rc::make_mut(data);
                data.character_data = self
                    .characters
                    .iter()
                    .map(|chara| {
                        data.character_data
                            .iter()
                            .find(|data| data.is_for(*chara))
                            .cloned()
                            .map(Result::Ok)
                            .unwrap_or_else(|| roster::load_data(*chara, ctx, &mut data.assets))
                    })
                    .collect::<GameResult<PlayerData<_>>>()?;
            }
            None => {
                let mut assets = Assets::new(ctx)?;

                let mut sounds = SoundList::new();

                for path in glob::glob("./resources/global/sounds/**/*.mp3")
                    .unwrap()
                    .filter_map(Result::ok)
                {
                    let sound = path
                        .file_stem()
                        .and_then(|item| item.to_str())
                        .and_then(|item| GlobalSound::from_str(item).ok());
                    if let Some(sound) = sound {
                        use rodio::source::Source;
                        let source = rodio::decoder::Decoder::new(std::io::BufReader::new(
                            std::fs::File::open(&path)?,
                        ))
                        .unwrap();
                        let source = rodio::buffer::SamplesBuffer::new(
                            source.channels(),
                            source.sample_rate(),
                            source.convert_samples().collect::<Vec<_>>(),
                        )
                        .buffered();

                        sounds.data.insert(sound, source);
                    }
                }

                let background = Stage::new(ctx, "/bg_14.png")?;
                let play_area = PlayArea {
                    width: background.width() as i32 * 100,
                };

                let graphics = load_global_graphics(ctx, &mut assets)?;

                let ui = UiElements {
                    font: graphics::Font::default(),
                    shield: ShieldUi {
                        active: graphics::Image::new(ctx, "/global/ui/lockout/active_shield.png")?,
                        disabled: graphics::Image::new(
                            ctx,
                            "/global/ui/lockout/disabled_shield.png",
                        )?,
                        passive: graphics::Image::new(
                            ctx,
                            "/global/ui/lockout/passive_shield.png",
                        )?,
                    },
                    roundstart: RoundStartUi {
                        gamestart: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/roundstart/game_start.json".into(),
                        )?,
                        action: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/roundstart/round_start.json".into(),
                        )?,
                        roundend: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/roundstart/round_end.json".into(),
                        )?,
                        round: [
                            AnimationGroup::load_from_json(
                                ctx,
                                &mut assets,
                                "./resources/global/ui/roundstart/last_round.json".into(),
                            )?,
                            AnimationGroup::load_from_json(
                                ctx,
                                &mut assets,
                                "./resources/global/ui/roundstart/round_one.json".into(),
                            )?,
                            AnimationGroup::load_from_json(
                                ctx,
                                &mut assets,
                                "./resources/global/ui/roundstart/round_two.json".into(),
                            )?,
                            AnimationGroup::load_from_json(
                                ctx,
                                &mut assets,
                                "./resources/global/ui/roundstart/round_three.json".into(),
                            )?,
                            AnimationGroup::load_from_json(
                                ctx,
                                &mut assets,
                                "./resources/global/ui/roundstart/round_three.json".into(),
                            )?,
                        ],
                    },
                    player: PlayerUi {
                        limit_bar: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/limit_bar.json".into(),
                        )?,
                        overlay: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/player_overlay.json".into(),
                        )?,
                        underlay: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/underlay.json".into(),
                        )?,
                        hp_bar: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/hp_bar.json".into(),
                        )?,
                        spirit_bar: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/spirit_bar.json".into(),
                        )?,
                        meter_bar: AnimationGroup::load_from_json(
                            ctx,
                            &mut assets,
                            "./resources/global/ui/meter_bar.json".into(),
                        )?,
                        underlay_round_windicator: graphics::Image::new(
                            ctx,
                            "/global/ui/underlay_windicator.png",
                        )?,
                        overlay_round_windicator: graphics::Image::new(
                            ctx,
                            "/global/ui/overlay_windicator.png",
                        )?,
                        round_windicator: graphics::Image::new(ctx, "/global/ui/windicator.png")?,
                    },
                    timer_backdrop: graphics::Image::solid(ctx, 80, graphics::BLACK)?,
                    fade_out_overlay: graphics::Image::solid(ctx, 1280, graphics::BLACK)?,
                };

                self.runtime_data = Some(Rc::new(RuntimeData {
                    character_data: self
                        .characters
                        .clone()
                        .map(|chara| roster::load_data(chara, ctx, &mut assets))
                        .transpose()?,
                    assets,
                    graphics,
                    sounds,
                    ui,
                    background,
                    play_area,
                }));
            }
        }

        Ok(())
    }
}

pub trait FromMatchSettings {
    fn from_settings(
        ctx: &mut Context,
        player_list: PlayerList,
        settings: MatchSettings,
    ) -> GameResult<Box<Self>>;
}

pub fn load_global_graphics(
    ctx: &mut Context,
    assets: &mut Assets,
) -> GameResult<HashMap<GlobalGraphic, AnimationGroup>> {
    let mut graphics = HashMap::new();
    let mut path = PathBuf::from("./resources/global/graphics");
    for graphic in GlobalGraphic::iter() {
        path.push(format!("{}.json", graphic));

        graphics.insert(
            graphic,
            AnimationGroup::load_from_json(ctx, assets, path.clone())?,
        );

        path.pop();
    }
    Ok(graphics)
}
