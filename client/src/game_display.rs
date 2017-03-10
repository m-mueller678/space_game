use game_manager::GameManager;
use key_manager::{Action, KeyManager};
use common::game::Game;
use common::graphics::RenderTarget as CommonRenderTrait;
use sfml::graphics::{RenderTarget, Color};
use sfml::window::event::Event;
use sfml::window::Key;
use std::cmp::{min, max};
use sfml::system::Clock;
use render::SfRender;
use common::serde_json;
use common::graphics::TransformRender;
use common::serde::de::Error as ErrorTrait;

type V2 = [f32; 2];

#[derive(Debug)]
pub enum RunResult {
    IoError(serde_json::Error),
    Quit,
    Win,
    Lose,
}


struct GameView<'a, 'b, 'c> {
    scroll: f32,
    draw_width: f32,
    selected: usize,
    game: &'a mut Game,
    player: usize,
    keys: &'b mut KeyManager,
    manager: &'c mut GameManager,
}

pub fn run(win: &mut SfRender,
           game: &mut Game,
           game_manager: &mut GameManager,
           player: usize,
           keys: &mut KeyManager)
           -> RunResult {
    let mut clock = Clock::new();
    let mut game = GameView {
        scroll: 0.,
        draw_width: 0.,
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
                    win.win.close();
                    return RunResult::Quit;
                },
                EventResult::IoError(e) => return RunResult::IoError(e)
            }
        }
        let dt = clock.restart().as_seconds();
        let mouse_pos = win.get_mouse_position();
        let m_x_pos = mouse_pos.x as f32 / win.get_size().x as f32;
        let in_win_y = mouse_pos.y >= 0 && mouse_pos.y < win.get_size().y as i32;
        let scroll_right = in_win_y && m_x_pos > 0.95 && m_x_pos < 1.;
        let scroll_left = in_win_y && m_x_pos < 0.05 && m_x_pos > 0.;
        if scroll_right ^ scroll_left {
            let direction = if scroll_right { 1. } else { -1. };
            scroll(&mut game, direction * dt * 5000., );
        }
        match game.manager.do_ticks(game.game) {
            Err(e) => return RunResult::IoError(e),
            Ok(true) => match game.game.winner() {
                Some(p) => if p == player {
                    return RunResult::Win
                } else {
                    return RunResult::Lose
                },
                None => return RunResult::IoError(serde_json::Error::custom("unexpected end from server")),
            },
            Ok(false) => {}
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
                Key::Right => {
                    scroll(game, 100.);
                },
                Key::Left => {
                    scroll(game, -100.);
                }
                k => {
                    match game.keys.get(&k) {
                        Some(&Action::SpawnShip(ship_id)) => {
                            if let Err(e) = game.manager.spawn_ship(game.player, game.selected, ship_id) {
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
    game.scroll += dist;
    bound_scroll(game);
}

fn bound_scroll(game: &mut GameView) {
    if game.scroll < 0. {
        game.scroll = 0.
    } else if game.scroll > game.game.size_x() as f32 - game.draw_width {
        game.scroll = game.game.size_x() as f32 - game.draw_width;
    }
}

fn resize(win: V2, game: &mut GameView) {
    game.draw_width = game.game.size_y() as f32 * win[0] / win[1];
    bound_scroll(game);
}

fn draw(win: &mut SfRender, game: &GameView) {
    let draw_height = game.game.size_y() as f32;
    let game_len = game.game.size_x() as f32;
    let y_scale = win.get_view().get_size().y / draw_height;
    win.clear(&Color::black());
    {
        let x_scale = win.get_view().get_size().x / game.draw_width;
        let mut render = TransformRender::new(win, move |(x, y)| (x * x_scale, y * y_scale));
        let (min_y, max_y) = game.game.lane_y_range(game.selected);
        let (min_y, max_y) = (min_y as f32, max_y as f32);
        let lane_height = max_y - min_y;
        render.draw_triangle(&[
            (0.1 * lane_height, min_y + lane_height * 0.25),
            (0.1 * lane_height, min_y + lane_height * 0.75),
            (0.3 * lane_height, min_y + 0.5 * lane_height)
        ], [0, 255, 255, 255]);
    }
    {
        let x_scale = win.get_view().get_size().x / if game.player == 0 { game.draw_width } else { -game.draw_width };
        let x_translate = if game.player == 0 { game.scroll } else { game_len - game.scroll };
        let mut render = TransformRender::new(win, move |(x, y)| ((x - x_translate) * x_scale, y * y_scale));
        game.game.draw(&mut render);
    }
    win.display();
}