use crate::input::Facing;
use crate::roster::generic_character::combo_state::ComboState;
use crate::typedefs::collision;
use crate::{
    character::state::components::{Flags, GlobalParticle, MoveType, ParticlePath},
    input::DirectedAxis,
};

pub fn handle_refacing(
    facing: &mut Facing,
    flags: &Flags,
    position: &collision::Vec2,
    other_player: collision::Int,
) {
    if flags.allow_reface {
        *facing = if position.x > other_player && *facing == Facing::Right {
            Facing::Left
        } else if position.x < other_player && *facing == Facing::Left {
            Facing::Right
        } else {
            *facing
        }
    }
}
pub fn handle_combo_state(
    current_combo: &mut Option<ComboState>,
    last_combo_state: &mut Option<(ComboState, usize)>,
    current_state_type: MoveType,
) {
    if !current_state_type.is_stun() {
        *current_combo = None;
    }

    if current_combo.is_some() {
        *last_combo_state = Some((current_combo.clone().unwrap(), 30));
    }
    if last_combo_state.is_some() && current_combo.is_none() {
        let (_, timer) = last_combo_state.as_mut().unwrap();
        *timer -= 1;
        if *timer == 0 {
            *last_combo_state = None;
        }
    }
}

use crate::assets::{Assets, UiProgress};
use crate::character::components::Properties;
use crate::game_match::UiElements;
use crate::typedefs::graphics;
use ggez::{Context, GameResult};

#[allow(clippy::too_many_arguments)]
pub fn draw_ui(
    ctx: &mut Context,
    assets: &Assets,
    ui: &UiElements,
    bottom_line: graphics::Matrix4,
    flipped: bool,
    wins: usize,
    first_to: usize,
    last_combo_state: &Option<(ComboState, usize)>,
    combo_text: &mut Option<ggez::graphics::Text>,
    health: i32,
    spirit_gauge: i32,
    meter: i32,
    lockout: i32,
    properties: &Properties,
) -> GameResult<()> {
    if let Some((combo, timer)) = last_combo_state {
        let text = format!(
            "{} hits\n{} damage\n{} limit",
            combo.hits,
            combo.total_damage,
            combo.available_limit.max(0)
        );

        if let Some(combo_text) = combo_text.as_mut() {
            if combo_text.fragments()[0].text != text {
                combo_text.fragments_mut()[0] = ggez::graphics::TextFragment::new(text);
            }
        } else {
            let mut temp = ggez::graphics::Text::new(text);
            temp.set_font(ui.font, ggez::graphics::Scale::uniform(30.0));
            temp.set_bounds(
                [400.0, 400.0],
                if flipped {
                    ggez::graphics::Align::Right
                } else {
                    ggez::graphics::Align::Left
                },
            );
            *combo_text = Some(temp);
        }

        let hits_text = combo_text.as_ref().unwrap();

        let width = 400.0;

        ggez::graphics::set_transform(
            ctx,
            graphics::Matrix4::new_translation(&graphics::Vec3::new(
                if flipped { -width + 290.0 } else { -290.0 },
                -150.0,
                0.0,
            )) * bottom_line
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(
                    if flipped { -1.0 } else { 1.0 },
                    1.0,
                    1.0,
                )),
        );
        ggez::graphics::apply_transformations(ctx)?;

        ggez::graphics::draw(
            ctx,
            hits_text,
            ggez::graphics::DrawParam::default().color(ggez::graphics::Color::new(
                1.0,
                1.0,
                1.0,
                *timer as f32 / 30.0,
            )),
        )?;

        let _lock = ggez::graphics::use_shader(ctx, &assets.ui_shader);

        // TODO replace with constant, (LIMIT_BAR_PIXEL_SIZE)
        let limit_ratio = combo.available_limit.max(0) as f32 / 200.0;

        assets.ui_shader.send(
            ctx,
            UiProgress {
                rate: limit_ratio,
                value: 1.0,
                alpha: (*timer as f32 / 30.0),
            },
        )?;

        ui.player.limit_bar.draw_at_time(
            ctx,
            assets,
            0,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(-185.0, -25.0, 0.0)),
        )?;
    }

    ui.player
        .underlay
        .draw_at_time(ctx, assets, 0, bottom_line)?;

    {
        let _lock = ggez::graphics::use_shader(ctx, &assets.ui_shader);
        assets.ui_shader.send(
            ctx,
            UiProgress {
                rate: health as f32 / properties.health as f32,
                value: 1.0,
                alpha: 1.0,
            },
        )?;

        ui.player.hp_bar.draw_at_time(
            ctx,
            assets,
            0,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(-35.0, -312.0, 0.0))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(-1.0, 1.0, 1.0)),
        )?;

        assets.ui_shader.send(
            ctx,
            UiProgress {
                rate: spirit_gauge as f32 / properties.max_spirit_gauge as f32,
                value: 1.0,
                alpha: 1.0,
            },
        )?;

        ui.player.spirit_bar.draw_at_time(
            ctx,
            assets,
            0,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(-140.0, 315.0, 0.0))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(1.0, 1.0, 1.0)),
        )?;

        assets.ui_shader.send(
            ctx,
            UiProgress {
                rate: meter as f32 / 200_00.0,
                value: 1.0,
                alpha: 1.0,
            },
        )?;

        ui.player.meter_bar.draw_at_time(
            ctx,
            assets,
            0,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(-170.0, 266.0, 0.0))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::Vec3::new(1.0, 1.0, 1.0)),
        )?;
    }
    ui.player
        .overlay
        .draw_at_time(ctx, assets, 0, bottom_line)?;

    // draw shield graphics

    ggez::graphics::set_transform(
        ctx,
        bottom_line * graphics::Matrix4::new_translation(&graphics::Vec3::new(-45.0, 230.0, 0.0)),
    );
    ggez::graphics::apply_transformations(ctx)?;
    let shield = if lockout > 0 {
        &ui.shield.disabled
    } else if meter >= 50_00 {
        &ui.shield.active
    } else {
        &ui.shield.passive
    };
    ggez::graphics::draw(ctx, shield, ggez::graphics::DrawParam::default())?;

    // draw round win markers

    for idx in 0..first_to {
        ggez::graphics::set_transform(
            ctx,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                    191.0 - idx as f32 * 25.0,
                    -279.0,
                    0.0,
                )),
        );
        ggez::graphics::apply_transformations(ctx)?;
        ggez::graphics::draw(
            ctx,
            &ui.player.underlay_round_windicator,
            ggez::graphics::DrawParam::default(),
        )?;
        if idx < wins {
            ggez::graphics::set_transform(
                ctx,
                bottom_line
                    * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                        191.0 - idx as f32 * 25.0,
                        -279.0,
                        0.0,
                    )),
            );
            ggez::graphics::apply_transformations(ctx)?;
            ggez::graphics::draw(
                ctx,
                &ui.player.round_windicator,
                ggez::graphics::DrawParam::default(),
            )?;
        }
        ggez::graphics::set_transform(
            ctx,
            bottom_line
                * graphics::Matrix4::new_translation(&graphics::Vec3::new(
                    190.0 - idx as f32 * 25.0,
                    -280.0,
                    0.0,
                )),
        );
        ggez::graphics::apply_transformations(ctx)?;
        ggez::graphics::draw(
            ctx,
            &ui.player.overlay_round_windicator,
            ggez::graphics::DrawParam::default(),
        )?;
    }

    Ok(())
}
use crate::typedefs::collision::IntoGraphical;
pub fn get_transform(
    world: graphics::Matrix4,
    offset: collision::Vec2,
    position: collision::Vec2,
    facing: Facing,
) -> graphics::Matrix4 {
    world
        * graphics::Matrix4::new_translation(&graphics::up_dimension(position.into_graphical()))
        * graphics::Matrix4::new_translation(&graphics::up_dimension(
            facing.fix_graphics(-offset.into_graphical()),
        ))
        * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
            facing.graphics_multiplier(),
        ))
}
use crate::graphics::particle::Particle;
use std::collections::HashMap;
use std::hash::Hash;

use super::extra_data::ExtraData;

pub fn draw_particles<K: Hash + Eq>(
    ctx: &mut Context,
    assets: &Assets,
    world: graphics::Matrix4,
    local_particles: &HashMap<K, Particle>,
    global_particles: &HashMap<GlobalParticle, Particle>,
    particle_list: &[(usize, collision::Vec2, ParticlePath<K>)],
) -> GameResult<()> {
    for (frame, position, id) in particle_list {
        let particle = id.get(local_particles, global_particles);

        particle.draw_at_time(
            ctx,
            assets,
            *frame,
            world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    position.into_graphical(),
                )),
        )?;
    }

    Ok(())
}

pub fn handle_fly<MoveId: Eq>(
    move_id: MoveId,
    fly_start: MoveId,
    extra_data: &mut ExtraData,
) -> collision::Vec2 {
    if move_id == fly_start {
        let fly_dir = extra_data.unwrap_fly_direction();
        *extra_data = ExtraData::FlyDirection(fly_dir);
        let speed = match fly_dir {
            DirectedAxis::Forward => collision::Vec2::new(1_00, 0_00),
            DirectedAxis::UpForward => collision::Vec2::new(0_71, 0_71),
            DirectedAxis::DownForward => collision::Vec2::new(0_71, -0_71),
            DirectedAxis::Backward => collision::Vec2::new(-1_00, 0_00),
            DirectedAxis::UpBackward => collision::Vec2::new(-0_71, 0_71),
            DirectedAxis::DownBackward => collision::Vec2::new(-0_71, -0_71),
            DirectedAxis::Up => collision::Vec2::new(0_00, 1_00),
            DirectedAxis::Down => collision::Vec2::new(0_00, -1_00),
            _ => unreachable!(),
        };
        3 * speed / 4
    } else {
        collision::Vec2::zeros()
    }
}
