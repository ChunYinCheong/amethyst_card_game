use super::{action::Action, card::Card, player::Player, target_type::TargetType};

#[derive(Clone, Default)]
pub struct BoardResource {
    pub players: Vec<Player>,
    pub cards: Vec<Card>,
    pub actions: Vec<Action>,
    pub target_types: Vec<TargetType>,
}

impl BoardResource {
    pub fn add_player(&mut self, player: Player) -> usize {
        let rid = self.players.len();
        self.players.push(player);
        rid
    }
    pub fn add_card(&mut self, card: Card) -> usize {
        let rid = self.cards.len();
        self.cards.push(card);
        rid
    }
}
