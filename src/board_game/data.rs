use super::action::ActionType;

#[derive(Clone)]
pub struct BoardData {
    pub players: Vec<PlayerData>,
}

#[derive(Clone)]
pub struct PlayerData {
    pub name: String,
    pub hands: Vec<CardData>,
    pub fronts: [Option<CardData>; 5],
    pub centers: [Option<CardData>; 5],
    pub backs: [Option<CardData>; 5],
    pub graves: Vec<CardData>,
    pub decks: Vec<CardData>,
    pub player_card: CardData,
}

#[derive(Clone)]
pub struct CardData {
    pub name: String,
    pub attributes: Vec<usize>,
    pub actions: Vec<ActionData>,
    pub place_cost_hp: i8,
    pub place_cost_mp: i8,
    pub hp: i8,
    pub mp: i8,
    pub max_hp: i8,
    pub max_mp: i8,
    pub face_up: bool,
    pub texture: String,
}

#[derive(Clone)]
pub struct AttributeData {
    pub name: String,
}

#[derive(Clone)]
pub struct ActionData {
    pub name: String,
    pub cost_hp: i8,
    pub cost_mp: i8,
    pub target_types: Vec<TargetTypeData>,
    pub action_type: ActionType,
}

#[derive(Clone)]
pub struct TargetTypeData {
    pub alliance: bool,
    pub enemy: bool,
}
