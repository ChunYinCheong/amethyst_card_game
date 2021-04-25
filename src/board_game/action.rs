use super::{
    board::Board,
    board_game::{BoardGame, Target},
    board_resource::BoardResource,
    game_move::GameMove,
};

pub type ActionId = usize;
#[derive(Clone)]
pub struct Action {
    pub id: ActionId,
    pub name: String,
    pub cost_hp: i8,
    pub cost_mp: i8,
    pub targets: Vec<usize>,
    pub action_type: ActionType,

    pub card_id: usize,
}

impl Action {
    pub fn possible_moves(&self, board: &Board, board_resource: &BoardResource) -> Vec<GameMove> {
        {
            // Check enough hp/mp
            let action_card = &board_resource.cards[board.cards[self.card_id]];
            if action_card.hp < self.cost_hp || action_card.mp < self.cost_mp {
                return Vec::new();
            }
        }
        if self.targets.is_empty() {
            return vec![GameMove::DoAction {
                action: self.id,
                targets: Default::default(),
            }];
        }
        let possibles: Vec<Vec<Target>> = self
            .targets
            .iter()
            .map(|id| {
                board_resource.target_types[board.target_types[*id]]
                    .possible_targets(board, board_resource)
            })
            .collect();
        if possibles.iter().any(|v| v.is_empty()) {
            // No target
            return Vec::new();
        }
        let mut combination = vec![];
        for v in possibles {
            if combination.is_empty() {
                for t in v {
                    combination.push(vec![t]);
                }
            } else {
                let nc: Vec<Vec<Target>> = v
                    .iter()
                    .map(|t| {
                        combination.iter().map(move |c| {
                            let mut n = c.clone();
                            n.push(t.clone());
                            n
                        })
                        // .collect::<Vec<Target>>()
                    })
                    .flatten()
                    .collect();
                combination = nc;
            }
        }

        combination
            .iter()
            .map(|vt| GameMove::DoAction {
                action: self.id,
                targets: vt.clone(),
            })
            .collect()
    }

    pub fn heuristic_value(&self, board: &Board, board_game: &BoardResource) -> i32 {
        0
    }
}
#[derive(Clone)]
pub enum ActionType {
    Attack { damage: i8 },
    Heal { hp: i8 },
    Lullaby { duration: i8 },
    Toxin { damage: i8, duration: i8 },
    Bind { duration: i8 },
    DrawCard,
    ManaTransmission,
    Teleport,
    Burnout { mp: i8 },
    LastStand,

    Regenerate { hp: i8 },
    Counterattack { damage: i8 },
    Defend,
}
