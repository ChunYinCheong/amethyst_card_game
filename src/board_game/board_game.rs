use super::{
    action::Action,
    board::Board,
    board_resource::BoardResource,
    card::{Card, CardId, CardPosition},
    data::{ActionData, BoardData, CardData, PlayerData, TargetTypeData},
    game_move::GameMove,
    player::PlayerId,
    target_type::TargetType,
};
use crate::board_game::player::Player;

#[derive(Clone)]
pub enum Target {
    Player(PlayerId),
    Card(CardId),
}

#[derive(Clone)]
pub enum Animation {
    Init,
    PlaceCard(usize),
    StartTurn(usize),
    EndTTurn(usize),
    DoAction(usize),
    Attack(usize),
    Heal(usize),
    Die(usize),
    EndGame,
}
#[derive(Clone)]
pub struct Event {
    pub message: String,
    pub animation: Animation,
}

#[derive(Clone)]
pub struct History {
    pub board: Board,
    pub event: Event,
}

#[derive(Clone, Default)]
pub struct BoardGame {
    pub histories: Vec<History>,

    pub board_resource: BoardResource,
}

impl BoardGame {
    pub fn new(data: BoardData) -> Self {
        // Init data
        let mut board_game = BoardGame::default();
        let mut board = Board::default();
        for p in data.players {
            Self::read_player_data(&mut board_game.board_resource, &mut board, p);
        }

        let event = Event {
            message: String::from("Init"),
            animation: Animation::Init,
        };
        board_game.histories.push(History { board, event });

        // Start Turn

        board_game
    }
    fn read_player_data(
        board_game: &mut BoardResource,
        board: &mut Board,
        player_data: PlayerData,
    ) -> usize {
        let player_id = board.players.len();
        let player_card_id = Self::read_card_data(
            board_game,
            board,
            player_id,
            &player_data.player_card,
            CardPosition::Back,
            Some(2),
        );

        let mut player = Player {
            id: player_id,
            name: player_data.name,
            hands: Vec::new(),
            fronts: Default::default(),
            centers: Default::default(),
            backs: Default::default(),
            graves: Vec::new(),
            decks: Vec::new(),
            player_card_id,
        };

        player.backs[2] = Some(player_card_id);
        for c in player_data.decks {
            let card_id =
                Self::read_card_data(board_game, board, player_id, &c, CardPosition::Deck, None);
            player.decks.push(card_id);
        }
        for c in player_data.hands {
            let card_id =
                Self::read_card_data(board_game, board, player_id, &c, CardPosition::Hand, None);
            player.hands.push(card_id);
        }
        for (i, oc) in player_data.backs.iter().enumerate() {
            match oc {
                Some(c) => {
                    let card_id = Self::read_card_data(
                        board_game,
                        board,
                        player_id,
                        c,
                        CardPosition::Back,
                        Some(i),
                    );
                    player.backs[i] = Some(card_id);
                }
                None => (),
            }
        }
        for (i, oc) in player_data.centers.iter().enumerate() {
            match oc {
                Some(c) => {
                    let card_id = Self::read_card_data(
                        board_game,
                        board,
                        player_id,
                        c,
                        CardPosition::Center,
                        Some(i),
                    );
                    player.centers[i] = Some(card_id);
                }
                None => (),
            }
        }
        for (i, oc) in player_data.fronts.iter().enumerate() {
            match oc {
                Some(c) => {
                    let card_id = Self::read_card_data(
                        board_game,
                        board,
                        player_id,
                        c,
                        CardPosition::Front,
                        Some(i),
                    );
                    player.fronts[i] = Some(card_id);
                }
                None => (),
            }
        }
        let rid = board_game.players.len();
        board.players.push(rid);
        board_game.players.push(player);
        player_id
    }
    fn read_card_data(
        board_game: &mut BoardResource,
        board: &mut Board,
        player_id: usize,
        card_data: &CardData,
        card_position: CardPosition,
        position_index: Option<usize>,
    ) -> usize {
        let card_id = board.cards.len();
        let mut card = Card {
            id: card_id,
            name: card_data.name.clone(),
            attributes: Vec::new(),
            actions: Vec::new(),
            place_cost_hp: card_data.place_cost_hp,
            place_cost_mp: card_data.place_cost_mp,
            hp: card_data.hp,
            mp: card_data.mp,
            face_up: card_data.face_up,
            card_position: card_position,
            player_id,
            position_index,
            max_hp: card_data.max_hp,
            max_mp: card_data.max_mp,
            texture_path: card_data.texture.clone(),
            status_effects: Default::default(),
        };
        for c in &card_data.actions {
            let id = Self::read_action_data(board_game, board, &c, card_id);
            card.actions.push(id);
        }
        let rid = board_game.cards.len();
        board.cards.push(rid);
        board_game.cards.push(card);
        card_id
    }
    fn read_action_data(
        board_game: &mut BoardResource,
        board: &mut Board,
        action_data: &ActionData,
        card_id: usize,
    ) -> usize {
        let action_id = board.actions.len();
        let mut action = Action {
            id: action_id,
            name: action_data.name.clone(),
            cost_hp: action_data.cost_hp,
            cost_mp: action_data.cost_mp,
            targets: Vec::new(),
            action_type: action_data.action_type.clone(),
            card_id,
        };
        for c in &action_data.target_types {
            let id = Self::read_target_type_data(board_game, board, &c, action_id);
            action.targets.push(id);
        }
        let rid = board_game.actions.len();
        board.actions.push(rid);
        board_game.actions.push(action);
        action_id
    }
    fn read_target_type_data(
        board_game: &mut BoardResource,
        board: &mut Board,
        target_type_data: &TargetTypeData,
        action_id: usize,
    ) -> usize {
        let target_type_id = board.target_types.len();

        let mut target_type = TargetType {
            id: target_type_id,
            action_id,
            alliance: target_type_data.alliance,
            enemy: target_type_data.enemy,
        };
        let rid = board_game.target_types.len();
        board.target_types.push(rid);
        board_game.target_types.push(target_type);
        target_type_id
    }

    pub fn run(&mut self, m: &GameMove) {
        let mut change = m.run(
            &self.histories.last().unwrap().board,
            &mut self.board_resource,
        );
        self.histories.append(&mut change);
    }

    pub fn current_board(&self) -> &Board {
        &self.histories.last().unwrap().board
    }
}
