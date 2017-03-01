use super::game_manager::GameManager;
use super::key_manager::{Action, KeyManager};
use common::game::Game;
use sfml::graphics::*;
use sfml::window::event::Event;
use sfml::window::Key;
use sfml::system::Vector2f;
use std::cmp::{min, max};
use sfml::system::Clock;
use common::serde_json;

type V2 = [f32; 2];

#[derive(Debug)]
pub enum RunResult {
    IoError(serde_json::Error),
    Quit,
}


struct GameView<'a, 'b, 'c> {
    scroll: f32,
    view: View,
    selected: usize,
    game: &'a mut Game,
    player: usize,
    keys: &'b mut KeyManager,
    manager: &'c mut GameManager,
}

pub fn run(win: &mut RenderWindow,
           game: &mut Game,
           game_manager: &mut GameManager,
           player: usize,
           keys: &mut KeyManager)
           -> RunResult {
    let mut clock = Clock::new();
    let mut game = GameView {
        scroll: 0.,
        view: win.get_view(),
        selected: 0,
        game: game,
        player: player,
        keys: keys,
        manager: game_manager,
    };
    resize([win.get_size().x as f32, win.get_size().y as f32], &mut game);
    while win.is_open() {
        for evt in win.events() {
            match handle_event(&mut game, evt) {
                EventResult::None => {},
                EventResult::Closed => {
                    win.close();
                    return RunResult::Quit;
                },
                EventResult::IoError(e) => return RunResult::IoError(e)
            }
        }
        let dt = clock.restart().as_seconds();
        let mouse_pos = win.get_mouse_position();
        let m_x_pos = mouse_pos.x as f32 / win.get_size().x as f32;
        let in_win_y = mouse_pos.y >= 0 && mouse_pos.y < win.get_size().y as i32;
        let scroll_right = Key::Right.is_pressed() || (in_win_y && m_x_pos > 0.95 && m_x_pos < 1.);
        let scroll_left = Key::Left.is_pressed() || (in_win_y && m_x_pos < 0.05 && m_x_pos > 0.);
        if scroll_right ^ scroll_left {
            let direction = if scroll_right { 1. } else { -1. };
            scroll(&mut game, direction * dt * 5000.);
        }
        if let Err(e) = game.manager.do_ticks(game.game) {
            return RunResult::IoError(e);
        }
        draw(win, &game);
        win.display();
    }
    RunResult::Quit
}

enum EventResult {
    None,
    Closed,
    IoError(serde_json::Error),
}

fn handle_event(game: &mut GameView, evt: Event) -> EventResult {
    match evt {
        Event::Resized { width, height } => {
            resize([width as f32, height as f32], game);
        },
        Event::KeyPressed { code, .. } => {
            match code {
                Key::Down => {
                    game.selected = min(game.selected + 1, game.game.lane_count() - 1);
                },
                Key::Up => {
                    game.selected = game.selected.saturating_sub(1);
                },
                k => {
                    match game.keys.get(&k) {
                        Some(&Action::SpawnShip(ship_id)) => {
                            if let Err(e) = game.manager.spawn_ship(game.player, ship_id, game.selected) {
                                return EventResult::IoError(e);
                            }
                        }
                        None => {}
                    }
                }
            }
        },
        Event::MouseWheelMoved { delta, .. } => {
            if Key::LShift.is_pressed() || Key::RShift.is_pressed() {
                scroll(game, -delta as f32 * 300.);
            } else {
                game.selected = min(max(game.selected as i32 - delta, 0) as usize, game.game.lane_count() - 1);
            }
        },
        Event::Closed => {
            return EventResult::Closed;
        },
        _ => {}
    }
    EventResult::None
}

fn scroll(game: &mut GameView, dist: f32) {
    game.scroll = (game.scroll + dist).max(0.).min(game.game.size_x() as f32 - game.view.get_size().x.abs());
    set_view_pos(game);
}

fn set_view_pos(game: &mut GameView) {
    let view_size = game.view.get_size();
    if game.player == 0 {
        game.view.set_center2f(
            game.scroll + view_size.x.abs() / 2.,
            view_size.y / 2.);
    } else {
        game.view.set_center2f(
            game.game.size_x() as f32 - game.scroll - view_size.x.abs() / 2.,
            view_size.y / 2.);
    }
}

fn resize(win: V2, game: &mut GameView) {
    let draw_h = game.game.size_y() as f32;
    let draw_w = draw_h * win[0] / win[1];
    game.scroll = game.scroll.max(0.).min(game.game.size_x() as f32 - draw_w);
    game.view.set_size2f(if game.player == 0 { draw_w } else { -draw_w }, draw_h);
    set_view_pos(game);
}

fn draw(win: &mut RenderWindow, game: &GameView) {
    win.set_view(&game.view);
    win.clear(&Color::black());
    {
        let lane_len = game.game.size_x() as f32 / 4.;
        let y_range = game.game.lane_y_range(game.selected);
        let lane_height = y_range.1 as f32 - y_range.0 as f32;
        let y_start = y_range.0 as f32 + lane_height * 0.2;
        let y_end = y_range.1 as f32 - lane_height * 0.2;
        let (x_start, x_end) = if game.player == 0 {
            (0., lane_len)
        } else {
            (game.game.size_x() as f32, game.game.size_x() as f32 - lane_len)
        };
        let col1 = Color::new_rgba(0, 255, 255, 64);
        let col2 = Color::new_rgba(0, 255, 255, 0);
        let lane_ver = [
            Vertex::new_with_pos_color(&Vector2f::new(x_start, y_start), &col1),
            Vertex::new_with_pos_color(&Vector2f::new(x_end, y_start), &col2),
            Vertex::new_with_pos_color(&Vector2f::new(x_end, y_end), &col2),
            Vertex::new_with_pos_color(&Vector2f::new(x_start, y_end), &col1),
        ];
        win.draw_primitives(&lane_ver, PrimitiveType::sfQuads, &mut RenderStates::default());
    }
    game.game.draw(win);
}