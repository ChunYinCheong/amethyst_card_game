use crate::board_game::data::{ActionData, BoardData, CardData, PlayerData, TargetTypeData};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{
    battle::Battle,
    board_game::{action::ActionType, board_game::BoardGame},
    board_texture::BoardTexture,
};
use amethyst::{assets::ProgressCounter, prelude::*};

#[derive(Default)]
pub struct LoadBattle {
    progress_counter: ProgressCounter,
}

impl SimpleState for LoadBattle {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut board_texture = BoardTexture::default();

        board_texture.load(
            data.world,
            "board_game/card/paper_shield.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/wood_shield.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/iron_shield.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/advanced_shield.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/paper_sword.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/sword.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/heal_gun.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/you.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/enemy.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/card/empty.png",
            &mut self.progress_counter,
        );
        board_texture.load(
            data.world,
            "board_game/status_effect/suppression.png",
            &mut self.progress_counter,
        );

        data.world.insert(board_texture);

        let paper_shield = CardData {
            name: String::from("Paper Shield"),
            attributes: Vec::new(),
            actions: vec![],
            place_cost_hp: 0,
            place_cost_mp: 1,
            hp: 1,
            mp: 0,
            max_hp: 1,
            max_mp: 0,
            face_up: false,
            texture: "board_game/card/paper_shield.png".to_string(),
        };
        let wood_shield = CardData {
            name: String::from("Wood Shield"),
            attributes: Vec::new(),
            actions: vec![],
            place_cost_hp: 0,
            place_cost_mp: 2,
            hp: 2,
            mp: 0,
            max_hp: 2,
            max_mp: 0,
            face_up: false,
            texture: "board_game/card/wood_shield.png".to_string(),
        };
        let iron_shield = CardData {
            name: String::from("Iron Shield"),
            attributes: Vec::new(),
            actions: vec![],
            place_cost_hp: 0,
            place_cost_mp: 3,
            hp: 3,
            mp: 0,
            max_hp: 3,
            max_mp: 0,
            face_up: false,
            texture: "board_game/card/iron_shield.png".to_string(),
        };
        let advanced_shield = CardData {
            name: String::from("Advanced Shield"),
            attributes: Vec::new(),
            actions: vec![ActionData {
                name: "Heal".to_string(),
                cost_hp: 0,
                cost_mp: 1,
                target_types: vec![],
                action_type: ActionType::Heal { hp: 1 },
            }],
            place_cost_hp: 0,
            place_cost_mp: 4,
            hp: 3,
            mp: 0,
            max_hp: 3,
            max_mp: 1,
            face_up: false,
            texture: "board_game/card/advanced_shield.png".to_string(),
        };

        let paper_sword = CardData {
            name: String::from("Paper Sword"),
            attributes: Vec::new(),
            actions: vec![ActionData {
                name: "Attack".to_string(),
                cost_hp: 0,
                cost_mp: 1,
                target_types: vec![TargetTypeData {
                    alliance: false,
                    enemy: true,
                }],
                action_type: ActionType::Attack { damage: 1 },
            }],
            place_cost_hp: 0,
            place_cost_mp: 2,
            hp: 1,
            mp: 0,
            max_hp: 1,
            max_mp: 1,
            face_up: false,
            texture: "board_game/card/paper_sword.png".to_string(),
        };
        let sword = CardData {
            name: String::from("Sword"),
            attributes: Vec::new(),
            actions: vec![ActionData {
                name: "Attack".to_string(),
                cost_hp: 0,
                cost_mp: 2,
                target_types: vec![TargetTypeData {
                    alliance: false,
                    enemy: true,
                }],
                action_type: ActionType::Attack { damage: 2 },
            }],
            place_cost_hp: 0,
            place_cost_mp: 2,
            hp: 1,
            mp: 0,
            max_hp: 1,
            max_mp: 2,
            face_up: false,
            texture: "board_game/card/sword.png".to_string(),
        };

        let heal_gun = CardData {
            name: String::from("Heal Gun"),
            attributes: Vec::new(),
            actions: vec![ActionData {
                name: "Heal".to_string(),
                cost_hp: 0,
                cost_mp: 1,
                target_types: vec![TargetTypeData {
                    alliance: true,
                    enemy: false,
                }],
                action_type: ActionType::Heal { hp: 1 },
            }],
            place_cost_hp: 0,
            place_cost_mp: 2,
            hp: 1,
            mp: 0,
            max_hp: 1,
            max_mp: 1,
            face_up: false,
            texture: "board_game/card/heal_gun.png".to_string(),
        };

        let decks = vec![
            paper_shield.clone(),
            paper_shield.clone(),
            paper_shield.clone(),
            wood_shield.clone(),
            wood_shield.clone(),
            wood_shield.clone(),
            iron_shield.clone(),
            iron_shield.clone(),
            iron_shield.clone(),
            advanced_shield.clone(),
            advanced_shield.clone(),
            advanced_shield.clone(),
            paper_sword.clone(),
            paper_sword.clone(),
            paper_sword.clone(),
            sword.clone(),
            sword.clone(),
            sword.clone(),
            heal_gun.clone(),
            heal_gun.clone(),
            heal_gun.clone(),
        ];
        let board_data = BoardData {
            players: vec![
                PlayerData {
                    name: String::from("You"),
                    hands: vec![paper_shield.clone(), paper_sword.clone()],
                    fronts: [
                        Some(paper_shield.clone()),
                        None,
                        Some(paper_shield.clone()),
                        None,
                        Some(paper_shield.clone()),
                    ],
                    centers: [
                        None,
                        Some(paper_sword.clone()),
                        None,
                        Some(paper_sword.clone()),
                        None,
                    ],
                    backs: Default::default(),
                    graves: Default::default(),
                    decks: {
                        let mut rng = thread_rng();
                        let mut d = decks.clone();
                        d.shuffle(&mut rng);
                        d
                    },
                    player_card: CardData {
                        name: "You".to_string(),
                        attributes: Default::default(),
                        actions: vec![
                            ActionData {
                                name: "Bind".to_string(),
                                cost_hp: 0,
                                cost_mp: 1,
                                target_types: vec![TargetTypeData {
                                    alliance: false,
                                    enemy: true,
                                }],
                                action_type: ActionType::Bind { duration: 3 },
                            },
                            ActionData {
                                name: "Lullaby".to_string(),
                                cost_hp: 0,
                                cost_mp: 1,
                                target_types: vec![TargetTypeData {
                                    alliance: false,
                                    enemy: true,
                                }],
                                action_type: ActionType::Lullaby { duration: 3 },
                            },
                        ],
                        place_cost_hp: 0,
                        place_cost_mp: 0,
                        hp: 5,
                        mp: 5,
                        max_hp: 5,
                        max_mp: 5,
                        face_up: true,
                        texture: "board_game/card/you.png".to_string(),
                    },
                },
                PlayerData {
                    name: String::from("Enemy"),
                    hands: vec![paper_shield.clone(), paper_sword.clone()],
                    fronts: [
                        Some(paper_shield.clone()),
                        None,
                        Some(paper_shield.clone()),
                        None,
                        Some(paper_shield.clone()),
                    ],
                    centers: [
                        None,
                        Some(paper_sword.clone()),
                        None,
                        Some(paper_sword.clone()),
                        None,
                    ],
                    backs: Default::default(),
                    graves: Default::default(),
                    decks: {
                        let mut rng = thread_rng();
                        let mut d = decks.clone();
                        d.shuffle(&mut rng);
                        d
                    },
                    player_card: CardData {
                        name: "Enemy".to_string(),
                        attributes: Default::default(),
                        actions: Default::default(),
                        place_cost_hp: 0,
                        place_cost_mp: 0,
                        hp: 5,
                        mp: 5,
                        max_hp: 5,
                        max_mp: 5,
                        face_up: true,
                        texture: "board_game/card/enemy.png".to_string(),
                    },
                },
            ],
        };
        let board_game = BoardGame::new(board_data);
        data.world.insert(board_game);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(Battle::new()))
        } else {
            Trans::None
        }
    }
}
/*
#[derive(Serialize, Deserialize, Default)]
struct PlayerSetting {
    pub name: String,
    pub cards: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlayerSettingData {
    /// Early version only could damage HP.
    Version1 { name: String, cards: Vec<String> },
    // Add support for subtracting MP.
    // Version2 { hp_damage: u32, mp_damage: u32 },
}
type PlayerSettingHandle = Handle<PlayerSetting>;
impl Asset for PlayerSetting {
    const NAME: &'static str = "my_crate::PlayerSetting";
    // use `Self` if the type is directly serialized.
    type Data = PlayerSettingData;
    type HandleStorage = VecStorage<PlayerSettingHandle>;
}

impl ProcessableAsset for PlayerSetting {
    fn process(energy_blast_data: Self::Data) -> Result<ProcessingState<Self>, Error> {
        match energy_blast_data {
            PlayerSettingData::Version1 { name, cards } => Ok(ProcessingState::Loaded(Self {
                name,
                cards,
                ..Default::default()
            })), // PlayerSettingData::Version2 { hp_damage, mp_damage } => {
                 //     Ok(ProcessingState::Loaded(Self {
                 //         hp_damage,
                 //         mp_damage,
                 //     }))
                 // }
        }
    }
}

pub struct LoadCard {
    /// Tracks loaded assets.
    progress_counter: ProgressCounter,
    /// Handle to the player texture.
    player1_handle: Handle<PlayerSetting>,
    player2_handle: Handle<PlayerSetting>,
}

impl SimpleState for LoadCard {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let loader = &data.world.read_resource::<Loader>();
        self.player1_handle.marker
    }

    // fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    //     if self.progress_counter.is_complete() {
    //         Trans::Switch(Box::new(GameState {
    //             texture_handle: self.player1_handle.take().expect(
    //                 "Expected `texture_handle` to exist when \
    //                     `progress_counter` is complete.",
    //             ),
    //         }))
    //     } else {
    //         Trans::None
    //     }
    // }
}

#[derive(Serialize, Deserialize, Default)]
struct CardSetting {
    name: String,
    cards: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CardSettingData {
    /// Early version only could damage HP.
    Version1 { name: String, cards: Vec<String> },
    // Add support for subtracting MP.
    // Version2 { hp_damage: u32, mp_damage: u32 },
}
type CardSettingHandle = Handle<CardSetting>;
impl Asset for CardSetting {
    const NAME: &'static str = "my_crate::CardSetting";
    // use `Self` if the type is directly serialized.
    type Data = CardSettingData;
    type HandleStorage = VecStorage<CardSettingHandle>;
}

impl ProcessableAsset for CardSetting {
    fn process(energy_blast_data: Self::Data) -> Result<ProcessingState<Self>, Error> {
        match energy_blast_data {
            CardSettingData::Version1 { name, cards } => Ok(ProcessingState::Loaded(Self {
                name,
                cards,
                ..Default::default()
            })), // CardSettingData::Version2 { hp_damage, mp_damage } => {
                 //     Ok(ProcessingState::Loaded(Self {
                 //         hp_damage,
                 //         mp_damage,
                 //     }))
                 // }
        }
    }
}
 */
