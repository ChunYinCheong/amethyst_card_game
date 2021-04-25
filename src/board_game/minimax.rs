use crate::board_game::{
    board::Board, board_game::BoardGame, board_resource::BoardResource, game_move::GameMove,
};
use std::{cmp, time::Instant};

fn minimax(
    node: &mut MinimaxNode,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    res: &mut BoardResource,
    counter: &mut usize,
) -> (i32, Option<GameMove>) {
    *counter += 1;
    if depth == 0 || node.is_terminal(res) {
        return (node.get_heuristic_value(res), None);
    }

    node.explore_possible_move(res);
    if maximizing_player {
        let mut max_value = i32::MIN;
        let mut best_move = None;
        for pm in node.possible_move.as_ref().unwrap().iter() {
            let board = pm.run(&node.game_state, res).pop().unwrap().board;
            let mut child_node = MinimaxNode {
                game_state: board,
                possible_move: None,
            };
            let maximizing = child_node.is_maximizing();
            let (value, child_move) = minimax(
                &mut child_node,
                depth - 1,
                alpha,
                beta,
                maximizing,
                res,
                counter,
            );
            if max_value < value {
                max_value = value;
                alpha = cmp::max(alpha, value);
                best_move = Some(pm.clone());
            }
            if beta <= alpha {
                break;
            }
        }
        return (max_value, best_move);
    } else {
        let mut min_value = i32::MAX;
        let mut best_move = None;
        for pm in node.possible_move.as_ref().unwrap().iter() {
            let board = pm.run(&node.game_state, res).pop().unwrap().board;
            let mut child_node = MinimaxNode {
                game_state: board,
                possible_move: None,
            };
            let maximizing = child_node.is_maximizing();
            let (value, child_move) = minimax(
                &mut child_node,
                depth - 1,
                alpha,
                beta,
                maximizing,
                res,
                counter,
            );
            if min_value > value {
                min_value = value;
                beta = cmp::min(beta, value);
                best_move = Some(pm.clone());
            }
            if beta <= alpha {
                break;
            }
        }
        return (min_value, best_move);
    }
}

pub fn next_move(board_game: &BoardGame) -> GameMove {
    let now = Instant::now();

    let depth = if cfg!(debug_assertions) { 4 } else { 6 };

    let res = &mut board_game.board_resource.clone();
    let mut node = MinimaxNode {
        game_state: board_game.current_board().clone(),
        possible_move: None,
    };
    node.explore_possible_move(res);
    if let Some(possible_move) = &mut node.possible_move {
        if possible_move.is_empty() {
            panic!("No possible move!")
        }
        if possible_move.len() == 1 {
            return possible_move.pop().unwrap();
        }
    }
    let mut counter = 0;
    let maximizing = node.is_maximizing();
    let (value, best_move) = minimax(
        &mut node,
        depth,
        i32::MIN,
        i32::MAX,
        maximizing,
        res,
        &mut counter,
    );
    println!(
        "Depth: {3}, Count: {0}, Secs: {1}, Avg: {2}",
        counter,
        now.elapsed().as_secs(),
        1000 * (counter as u128) / now.elapsed().as_millis(),
        depth,
    );
    return best_move.unwrap();
}

struct MinimaxNode {
    game_state: Board,
    possible_move: Option<Vec<GameMove>>,
}

impl MinimaxNode {
    fn is_terminal(&self, res: &BoardResource) -> bool {
        return self.game_state.is_terminal(res);
    }

    fn get_heuristic_value(&self, res: &BoardResource) -> i32 {
        return self.game_state.heuristic_value(res);
    }

    fn explore_possible_move(&mut self, res: &mut BoardResource) {
        let moves = self.game_state.possible_moves(res);
        self.possible_move = Some(moves);
    }

    fn is_maximizing(&self) -> bool {
        return self.game_state.current_player_index != 0;
    }
}
