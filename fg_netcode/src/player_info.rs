use fg_datastructures::roster::RosterCharacter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerInfo {
    pub name: String,
    pub character: RosterCharacter,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        Self {
            name: "Fake Player".to_string(),
            character: RosterCharacter::default(),
        }
    }
}
