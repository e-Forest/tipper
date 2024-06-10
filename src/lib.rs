use std::fs;

use macroquad::{
    color::{Color, BLUE, GREEN, RED, WHITE, YELLOW},
    input::{
        get_char_pressed, is_key_down, is_key_pressed, is_key_released, is_mouse_button_down,
        mouse_position, KeyCode,
    },
    math::{vec2, Rect, Vec2},
    rand::RandomRange,
    shapes::{draw_circle, draw_rectangle},
    text::draw_text,
    time::get_frame_time,
    window::{screen_height, screen_width},
};

const MAX_WORD_SPEED: i32 = 10;

const SPAWN_TIMER_MAX: f32 = 5.;

#[derive(Default)]
pub struct Data {
    words_pool: Vec<String>,
    words: Vec<Word>,
    load: String,
    case_sensitive: bool,
    pub world_rect: Rect,
    spawn_timer: f32,
    pub ship: Ship,
}
impl Data {
    pub fn new() -> Self {
        let word_pool: Vec<String> = fs::read_to_string("words_pool.txt")
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect();

        let margin = 40.;
        let rx = margin;
        let ry = 0.;
        let rw = screen_width() - margin * 2.;
        let rh = screen_height() - margin * 2.;
        let spawn_rect = Rect::new(rx, ry, rw, rh);
        let ship = Ship::new(vec2(50., 50.));

        Self {
            words_pool: word_pool,
            world_rect: spawn_rect,
            ship,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        for word in &mut self.words {
            word.update();
            word.draw(&self.load);
            if word.pos.distance(self.ship.pos) < word.radius() + self.ship.radius {
                self.ship.radius -= 1. + word.text.len() as f32;
                word.alive = false;
            }
        }
        self.draw_world_rect();
        self.words.retain(|x| x.alive == true);
        self.ship.update(self.world_rect);
        self.update_load();
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0. {
            self.add_words(5, screen_width());
            self.spawn_timer = SPAWN_TIMER_MAX;
        }
        self.draw_load(self.world_rect);
        self.ship.draw();
    }

    pub fn add_words(&mut self, max: usize, add_x: f32) {
        let mut count = max;
        if count >= self.words_pool.len() {
            count = self.words_pool.len() - 1;
        }
        loop {
            let dice_empty = RandomRange::gen_range(0, 3);
            let pos = vec2(
                RandomRange::gen_range(0, 300) as f32 + add_x,
                RandomRange::gen_range(self.world_rect.y, self.world_rect.h) as f32,
            );

            if dice_empty == 0 {
                let dice_word = RandomRange::gen_range(0, self.words_pool.len() - 1);
                let Some(w) = self.words_pool.get(dice_word) else {
                    continue;
                };
                if self.case_sensitive {
                    self.words.push(Word::new(pos, w.clone()));
                } else {
                    self.words.push(Word::new(pos, w.to_lowercase().clone()));
                }
            } else {
                // let word = Word::new(pos,String::new() );
                self.words.push(Word::new(pos, String::new()));
            }
            if self.words.len() > count {
                break;
            }
        }
    }

    pub fn add_character_to_load(&mut self, character: char) {
        self.load.push(character);
    }

    pub fn clear_load(&mut self) {
        self.load = String::new();
    }

    pub fn shoot_load(&mut self) {
        for word in self
            .words
            .iter_mut()
            .filter(|x| is_match(&x.text, &self.load))
        {
            word.alive = false;
        }
    }
    pub fn draw_world_rect(&self) {
        let r = self.world_rect;
        draw_rectangle(0., r.h, screen_width(), screen_height() - r.h, WHITE);
    }

    pub fn draw_load(&self, border: Rect) {
        draw_text(&format!("'{}'", &self.load), 300., border.h + 40., 40., RED);
    }

    pub fn update_load(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.shoot_load();
            self.clear_load();
        } else if is_key_released(KeyCode::Backspace) {
            self.clear_load();
        } else if let Some(c) = get_char_pressed() {
            if !c.is_whitespace() {
                self.load.push(c);
            }
        }
        self.load = self.load.trim().to_string();
    }
}
pub fn is_match(w1: &str, w2: &str) -> bool {
    if w1 == w2 {
        true
    } else {
        false
    }
}
pub struct Word {
    pos: Vec2,
    text: String,
    speed: f32,
    alive: bool,
}

impl Word {
    pub fn new(pos: Vec2, text: String) -> Self {
        let mut speed = RandomRange::gen_range(1, MAX_WORD_SPEED) as f32;
        if text.is_empty() {
            speed *= 5.
        }
        Self {
            pos,
            text,
            speed,
            alive: true,
        }
    }
    pub fn draw(&self, load: &str) {
        let heigth = 20.;
        let x = self.pos.x;
        let y = self.pos.y;
        draw_circle(x, y, self.radius(), RED);
        draw_text(&self.text, x, y, heigth, WHITE);
        if self.text.starts_with(load) {
            draw_text(load, x, y, heigth, GREEN);
        }
    }

    fn update(&mut self) {
        self.pos.x -= self.speed * get_frame_time();
    }
    fn radius(&self) -> f32 {
        let mut result = 3. * self.text.len() as f32;
        if result < 3. {
            result = 3.;
        }
        result
    }
}

fn get_move_direction() -> Vec2 {
    let mut result = Vec2::ZERO;
    if is_key_down(KeyCode::Up) {
        result.y -= 1.
    }
    if is_key_down(KeyCode::Down) {
        result.y += 1.
    }
    if is_key_down(KeyCode::Left) {
        result.x -= 1.
    }
    if is_key_down(KeyCode::Right) {
        result.x += 1.
    }
    if result != Vec2::ZERO {
        result = result.normalize()
    }
    result
}

#[derive(Debug, Default)]
pub struct Ship {
    pos: Vec2,
    pub radius: f32,
    color: Color,
    speed: f32,
}
impl Ship {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            radius: 20.,
            color: BLUE,
            speed: 100.,
        }
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
        let m = mouse_position();
        draw_circle(m.0, m.1, 3., YELLOW);
    }
    pub fn update(&mut self, move_border: Rect) {
        let mut dir = get_move_direction();
        let dt = get_frame_time();
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let m = mouse_position();
            let mv = vec2(m.0, m.1);
            dir = (mv - self.pos).normalize()
        }
        self.pos += dir * self.speed * dt;
        if self.pos.y > move_border.h {
            self.pos.y = move_border.h
        }
    }
}
