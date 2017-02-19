use super::game_timer::GameTimer;
use common::game::Game;
use sfml::graphics::*;
use sfml::window::event::Event;
use sfml::system::Vector2f;
use std::cmp::min;

type V2 = [f32; 2];

pub fn run(win: &mut RenderWindow, game: &mut Game, game_timer: &mut GameTimer) {
    let mut scroll = 0.;
    let mut view = win.get_view();
    let mut selected = 0;
    resize([win.get_size().x as f32, win.get_size().y as f32],
           [game.size_x() as f32, game.size_y() as f32],
           &mut scroll, &mut view);
    loop {
        for evt in win.events() {
            match evt {
                Event::Resized { width, height } => {
                    resize([width as f32, height as f32],
                           [game.size_x() as f32, game.size_y() as f32],
                           &mut scroll, &mut view)
                },
                Event::KeyPressed { code, .. } => {
                    match code {
                        _ => {}
                    }
                },
                Event::Closed => {
                    win.close();
                    return;
                }
                _ => {}
            }
        }
        game_timer.do_ticks(game);
        draw(win, game, scroll, selected);
        win.display();
    }
}

fn resize(win: V2, game: V2, scroll: &mut f32, view: &mut View) {
    let mut draw_h = game[1];
    let mut draw_w = draw_h * win[0] / win[1];
    if draw_w > game[0] {
        draw_w = game[0];
        draw_h = draw_w * win[1] / win[0];
        *scroll = 0.;
    } else {
        *scroll = scroll.min(game[0] - draw_w);
    }
    view.set_center2f(*scroll + draw_w / 2., draw_h / 2.);
    view.set_size2f(draw_w, draw_h);
}

fn draw(win: &mut RenderWindow, game: &Game, scroll: f32, selected: usize) {
    let win_size = win.get_size();
    let h = game.size_y() as f32;
    let w = h * win_size.x as f32 / win_size.y as f32;
    let view = View::new_from_rect(&FloatRect::new(scroll, 0., w, h)).unwrap();
    win.set_view(&view);
    win.clear(&Color::black());
    let lane_len = min(game.size_y(), 50000) as f32;
    let y_range = game.lane_y_range(selected);
    let lane_ver = [
        Vertex::new_with_pos_color(&Vector2f::new(0., y_range.0 as f32),
                                   &Color::new_rgba(0, 255, 255, 64)),
        Vertex::new_with_pos_color(&Vector2f::new(lane_len, (y_range.0 as f32 + y_range.1 as f32) / 2.),
                                   &Color::new_rgba(0, 255, 255, 64)),
        Vertex::new_with_pos_color(&Vector2f::new(0., y_range.1 as f32),
                                   &Color::new_rgba(0, 255, 255, 64)),
    ];
    win.draw_primitives(&lane_ver, PrimitiveType::sfTriangles, &mut RenderStates::default());
    game.draw(win);
}