use std::collections::{hash_map::Keys, HashMap};

use fg_controller::backend::{Button, ControllerBackend, ControllerState, Event, EventType};
use fg_input::axis::Axis;
use sdl2::{
    controller::GameController,
    controller::{Axis as SdlAxis, Button as SdlButton},
    event::Event as SdlEvent,
    EventPump, GameControllerSubsystem, Sdl,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControllerId(u32);

pub struct SdlController {
    _sdl: Sdl,
    controller_subsystem: GameControllerSubsystem,
    events: EventPump,
    controllers: HashMap<ControllerId, GameController>,
    active_controller: Option<ControllerId>,
}

const TRIGGER_DEAD_ZONE: i16 = 15_000;
const STICK_DEAD_ZONE: i16 = i16::MAX / 10 * 7;

impl SdlController {
    pub fn new() -> Result<Self, String> {
        let _sdl = sdl2::init()?;
        let controllers = _sdl.game_controller()?;
        let events = _sdl.event_pump()?;
        Ok(Self {
            _sdl,
            controller_subsystem: controllers,
            events,
            controllers: Default::default(),
            active_controller: None,
        })
    }
}

pub struct SdlControllerIter<'a> {
    internal: Keys<'a, ControllerId, GameController>,
}

impl<'a> Iterator for SdlControllerIter<'a> {
    type Item = ControllerId;
    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().copied()
    }
}

fn value_to_x_axis(value: i16, dead_zone: i16) -> Axis {
    if value >= dead_zone {
        Axis::Right
    } else if value <= -dead_zone {
        Axis::Left
    } else {
        Axis::Neutral
    }
}
fn value_to_y_axis(value: i16, dead_zone: i16) -> Axis {
    if value >= dead_zone {
        Axis::Up
    } else if value <= -dead_zone {
        Axis::Down
    } else {
        Axis::Neutral
    }
}

impl<'this> ControllerBackend<'this> for SdlController {
    type ControllerId = ControllerId;

    type ControllerIter = SdlControllerIter<'this>;

    fn poll(&mut self) {
        while let Some(event) = self.events.poll_event() {
            match event {
                SdlEvent::ControllerButtonDown { which, .. }
                | SdlEvent::ControllerButtonUp { which, .. }
                | SdlEvent::ControllerAxisMotion { which, .. }
                | SdlEvent::ControllerDeviceRemapped { which, .. } => {
                    self.active_controller = Some(ControllerId(which));
                }
                SdlEvent::ControllerDeviceAdded { which, .. } => {
                    let controller = self.controller_subsystem.open(which).unwrap();
                    let id = ControllerId(controller.instance_id());
                    self.controllers.insert(id, controller);
                }
                SdlEvent::ControllerDeviceRemoved { which, .. } => {
                    let id = ControllerId(which);
                    self.controllers.remove(&id);
                }
                _ => self.poll(),
            }
        }
    }

    fn controllers(&'this self) -> Self::ControllerIter {
        SdlControllerIter {
            internal: self.controllers.keys(),
        }
    }

    fn current_state(&self, id: &Self::ControllerId) -> ControllerState {
        let controller = self.controllers.get(id).unwrap();

        let mut state = ControllerState::default();
        state[Button::A] = controller.button(SdlButton::A);
        state[Button::B] = controller.button(SdlButton::B);
        state[Button::X] = controller.button(SdlButton::X);
        state[Button::Y] = controller.button(SdlButton::Y);
        state[Button::Start] = controller.button(SdlButton::Start);
        state[Button::Back] = controller.button(SdlButton::Back);
        state[Button::Guide] = controller.button(SdlButton::Guide);
        state[Button::L1] = controller.button(SdlButton::LeftShoulder);
        state[Button::L2] = controller.axis(SdlAxis::TriggerLeft) > TRIGGER_DEAD_ZONE;
        state[Button::LeftStick] = controller.button(SdlButton::LeftStick);
        state[Button::R1] = controller.button(SdlButton::RightShoulder);
        state[Button::R2] = controller.axis(SdlAxis::TriggerRight) > TRIGGER_DEAD_ZONE;
        state[Button::RightStick] = controller.button(SdlButton::RightStick);

        state.left_stick = value_to_x_axis(controller.axis(SdlAxis::LeftX), STICK_DEAD_ZONE)
            + value_to_y_axis(controller.axis(SdlAxis::LeftY), STICK_DEAD_ZONE);

        state.right_stick = value_to_x_axis(controller.axis(SdlAxis::RightX), STICK_DEAD_ZONE)
            + value_to_y_axis(controller.axis(SdlAxis::RightY), STICK_DEAD_ZONE);

        state
    }

    fn active_controller(&self) -> Option<Self::ControllerId> {
        self.active_controller
    }
}
