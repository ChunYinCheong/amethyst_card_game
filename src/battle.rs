mod ui_input;

use crate::board_game::board::Board;
use crate::pause::PauseMenuState;

use amethyst::{
    core::Time,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    input::{is_close_requested, is_key_down, InputEvent},
    prelude::*,
    shred::{Read, Write},
    shrev::EventChannel,
    ui::{UiImage, UiText, UiTransform},
    winit::{MouseButton, VirtualKeyCode},
};

use crate::{
    board_game::{board_game::BoardGame, minimax},
    board_texture::BoardTexture,
    load_battle::LoadBattle,
    systems::BoardEvent,
};

mod action_container;
mod action_ui;
mod big_icon;
mod board_ui;
mod card_ui;
mod end_turn_button;
pub mod input_state;
mod log_button;
mod message_log;
mod overlay;

use ui_input::UiAndInput;

#[derive(Default)]
pub struct Battle {
    paused: bool,
}

impl Battle {
    pub fn new() -> Self {
        Battle {
            // board_game,
            ..Default::default()
        }
    }
}

impl SimpleState for Battle {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        // self.ui_and_input = Some(UiAndInput::new(world));

        let ui_and_input = Some(UiAndInput::new(world));
        world.insert(ui_and_input);
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = true;
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        self.paused = false;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // if let Some(board_ui) = &mut self.ui_and_input {
        //     board_ui.delete(data.world);
        // }
        data.world.exec(
            |(mut opt, entities): (Write<Option<UiAndInput>>, Entities)| {
                if let Some(ui) = opt.as_mut() {
                    ui.delete(&entities);
                }
            },
        );
    }

    fn handle_event(
        &mut self,
        state_data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Push] Pausing Game!");
                    Trans::Push(Box::new(PauseMenuState::default()))
                } else if is_key_down(&event, VirtualKeyCode::F5) {
                    log::info!("[Trans::Push] Refresh Game!");
                    Trans::Switch(Box::new(LoadBattle::default()))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                // log::info!(
                //     "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                //     ui_event
                // );
                state_data.world.exec(
                    |(
                        mut opt,
                        board_game,
                        mut ui_text,
                        mut ui_image,
                        mut ui_transform,
                        time,
                        mut event_channel,
                    ): (
                        Write<Option<UiAndInput>>,
                        Read<BoardGame>,
                        WriteStorage<UiText>,
                        WriteStorage<UiImage>,
                        WriteStorage<UiTransform>,
                        Read<Time>,
                        Write<EventChannel<BoardEvent>>,
                    )| {
                        if let Some(ui) = opt.as_mut() {
                            if let Some(m) = ui.handle_event(
                                &mut ui_text,
                                &mut ui_image,
                                &mut ui_transform,
                                ui_event,
                                &board_game,
                            ) {
                                event_channel.single_write(BoardEvent::PlayerMove(m));
                            }
                        }
                    },
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                // log::info!("Input Event detected: {:?}.", input);
                if let InputEvent::MouseButtonPressed(MouseButton::Right) = input {
                    state_data.world.exec(
                        |(
                            mut opt,
                            board_game,
                            mut ui_text,
                            mut ui_image,
                            mut ui_transform,
                            time,
                        ): (
                            Write<Option<UiAndInput>>,
                            Read<BoardGame>,
                            WriteStorage<UiText>,
                            WriteStorage<UiImage>,
                            WriteStorage<UiTransform>,
                            Read<Time>,
                        )| {
                            if let Some(ui) = opt.as_mut() {
                                ui.right_click(
                                    &mut ui_text,
                                    &mut ui_image,
                                    &mut ui_transform,
                                    &board_game,
                                );
                            }
                        },
                    );
                }
                Trans::None
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // it is important that the 'paused' field is actually pausing your game.
        // Make sure to also pause your running systems.
        if !self.paused {
            return world.exec(
                |(
                    mut opt,
                    board_game,
                    mut ui_text,
                    mut ui_image,
                    mut ui_transform,
                    time,
                    board_texture,
                ): (
                    Write<Option<UiAndInput>>,
                    Read<BoardGame>,
                    WriteStorage<UiText>,
                    WriteStorage<UiImage>,
                    WriteStorage<UiTransform>,
                    Read<Time>,
                    Read<BoardTexture>,
                )| {
                    if let Some(ui) = opt.as_mut() {
                        return ui.update(
                            &board_game,
                            &mut ui_text,
                            &mut ui_image,
                            &mut ui_transform,
                            &time,
                            &board_texture,
                        );
                    }
                    Trans::None
                },
            );
            // if let Some(ui_input) = &mut self.ui_and_input {
            //     let board_game = (*world.fetch::<BoardGame>()).clone();
            //     ui_input.update(world, &board_game);
            // }
        }

        Trans::None
    }
}
