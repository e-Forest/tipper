use std::fs;

use macroquad::{
    color::{GREEN, RED, WHITE},
    input::{get_char_pressed, is_key_pressed, is_key_released, KeyCode},
    math::{vec2, Rect, Vec2},
    rand::RandomRange,
    shapes::draw_circle,
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
    spawn_rect: Rect,
    spawn_timer: f32,
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
        let ry = margin;
        let rw = screen_width() - margin * 2.;
        let rh = screen_height() - margin * 2.;
        let spawn_rect = Rect::new(rx, ry, rw, rh);

        Self {
            words_pool: word_pool,
            spawn_rect,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        for word in &mut self.words {
            word.draw(&self.load);
            word.update();
        }
        self.words.retain(|x| x.alive == true);
        self.draw_load();
        self.update_load();
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0. {
            self.add_words(5, screen_width());
            self.spawn_timer = SPAWN_TIMER_MAX;
        }
    }

    pub fn add_words(&mut self, max: usize, add_x: f32) {
        let mut count = max;
        if count >= self.words_pool.len() {
            count = self.words_pool.len() - 1;
        }
        loop {
            let dice = RandomRange::gen_range(0, self.words_pool.len() - 1);
            let Some(w) = self.words_pool.get(dice) else {
                continue;
            };
            let pos = vec2(
                RandomRange::gen_range(0, 300) as f32 + add_x,
                RandomRange::gen_range(self.spawn_rect.y, self.spawn_rect.h) as f32,
            );

            if self.case_sensitive {
                self.words.push(Word::new(pos, w.clone()));
            } else {
                self.words.push(Word::new(pos, w.to_lowercase().clone()));
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

    pub fn draw_load(&self) {
        draw_text(&format!("'{}'", &self.load), 300., 550., 40., RED);
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
        let speed = RandomRange::gen_range(1, MAX_WORD_SPEED) as f32;
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
        5. * self.text.len() as f32
    }
}
