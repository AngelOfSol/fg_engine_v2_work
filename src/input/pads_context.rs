pub use sdl2::controller::Button;
use sdl2::controller::GameController;
use sdl2::event::Event as SdlEvent;
use sdl2::GameControllerSubsystem;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub id: GamepadId,
    pub event: EventType,
}
#[derive(Debug, Copy, Clone)]
pub enum EventType {
    ButtonPressed(Button),
    ButtonReleased(Button),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GamepadId(u32);

impl std::fmt::Display for GamepadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct PadsContext {
    controllers: Vec<GameController>,
    events: Vec<Event>,
    subsystem: GameControllerSubsystem,
}

impl PadsContext {
    pub fn new(subsystem: GameControllerSubsystem) -> Self {
        Self {
            subsystem,
            events: Vec::new(),
            controllers: Vec::new(),
        }
    }

    pub fn gamepads<'a>(&'a self) -> impl Iterator<Item = (GamepadId, &'a GameController)> {
        self.controllers
            .iter()
            .map(|item| (GamepadId(item.instance_id()), item))
    }

    pub fn gamepad(&self, GamepadId(id): GamepadId) -> &GameController {
        self.controllers
            .iter()
            .find(|item| item.instance_id() == id)
            .unwrap()
    }

    pub fn handle(&mut self, event: SdlEvent) {
        match event {
            SdlEvent::ControllerDeviceAdded { which, .. } => {
                self.controllers.push(self.subsystem.open(which).unwrap())
            }
            SdlEvent::ControllerDeviceRemoved { which, .. } => {
                self.controllers.swap_remove(
                    self.controllers
                        .iter()
                        .position(|item| item.instance_id() == which)
                        .unwrap(),
                );
            }
            SdlEvent::ControllerButtonDown { which, button, .. } => {
                self.events.insert(
                    0,
                    Event {
                        id: GamepadId(which),
                        event: EventType::ButtonPressed(button),
                    },
                );
            }
            SdlEvent::ControllerButtonUp { which, button, .. } => {
                self.events.insert(
                    0,
                    Event {
                        id: GamepadId(which),
                        event: EventType::ButtonReleased(button),
                    },
                );
            }
            _ => (),
        }
    }
    pub fn next_event(&mut self) -> Option<Event> {
        self.events.pop()
    }
}
