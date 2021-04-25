use crate::board_game::{board_game::BoardGame, game_move::GameMove, minimax};
use amethyst::{
    core::{
        shrev::{EventChannel, ReaderId},
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::{Read, System, SystemData, World, Write},
    prelude::*,
};
use std::{thread, time::Instant};

pub enum BoardEvent {
    PlayerMove(GameMove),
}

#[derive(SystemDesc)]
#[system_desc(name(BoardSystemDesc))]
pub struct BoardSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<BoardEvent>,
}

impl BoardSystem {
    pub fn new(reader: ReaderId<BoardEvent>) -> Self {
        BoardSystem { reader }
    }
}

impl<'a> System<'a> for BoardSystem {
    type SystemData = (
        Read<'a, EventChannel<BoardEvent>>,
        Write<'a, BoardGame>,
        Read<'a, CallbackQueue>,
    );

    fn run(&mut self, (my_event_channel, mut board_game, callback): Self::SystemData) {
        for event in my_event_channel.read(&mut self.reader) {
            match event {
                BoardEvent::PlayerMove(gm) => {
                    board_game.run(gm);
                    if board_game.current_board().current_player_index == 1 {
                        let handle = callback.send_handle();
                        let board_game = board_game.clone();
                        thread::spawn(move || {
                            let now = Instant::now();
                            log::info!("Ai Start!");
                            let ai_move = minimax::next_move(&board_game);
                            log::info!("Ai End! Secs: {}", now.elapsed().as_secs());
                            handle
                                .send(Box::new(move |world: &mut World| {
                                    let mut event_channel =
                                        world.write_resource::<EventChannel<BoardEvent>>();
                                    event_channel
                                        .single_write(BoardEvent::PlayerMove(ai_move.clone()));
                                }))
                                .expect("Failed to add Callback to CallbackQueue.");
                        });
                    }
                }
            }
        }
    }
}
