use std::fs;

use macroquad::{
    color::{GREEN, RED, WHITE},
    input::{get_char_pressed, is_key_pressed, KeyCode},
    rand::RandomRange,
    text::draw_text,
};

#[derive(Default)]
pub struct Data {
    words_pool: Vec<String>,
    words: Vec<String>,
    load: String,
    case_sensitive: bool,
}
impl Data {
    pub fn new() -> Self {
        let word_pool: Vec<String> = fs::read_to_string("words_pool.txt")
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect();
        Self {
            words_pool: word_pool,
            ..Default::default()
        }
    }

    pub fn add_words(&mut self, max: usize) {
        let mut count = max;
        if count >= self.words_pool.len() {
            count = self.words_pool.len() - 1;
        }
        loop {
            let dice = RandomRange::gen_range(0, self.words_pool.len() - 1);
            let Some(w) = self.words_pool.get(dice) else {
                continue;
            };
            if !self.words.contains(&w) {
                if self.case_sensitive {
                    self.words.push(w.clone());
                } else {
                    self.words.push(w.to_lowercase().clone());
                }
                if self.words.len() > count {
                    break;
                }
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
        self.words.retain(|x| !is_match(x, &self.load));
        self.clear_load();
    }

    pub fn draw_words(&self) {
        let width = 200.;
        let heigth = 20.;
        let words_in_a_row = 4;
        for (i, word) in self.words.iter().enumerate() {
            let x = (i % words_in_a_row) as f32;
            draw_text(
                &word,
                x * width,
                heigth + (i / words_in_a_row) as f32 * heigth,
                heigth,
                WHITE,
            );
            if word.starts_with(&self.load.trim()) {
                draw_text(
                    &self.load,
                    x * width,
                    heigth + (i / words_in_a_row) as f32 * heigth,
                    heigth,
                    GREEN,
                );
            }
        }
    }

    pub fn draw_load(&self) {
        draw_text(&self.load, 300., 550., 30., RED);
    }

    pub fn update_load(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.shoot_load();
            self.clear_load();
        } else if is_key_pressed(KeyCode::Backspace) {
            self.clear_load();
        } else if let Some(c) = get_char_pressed() {
            if !c.is_whitespace() {
                self.load.push(c);
            }
        }
    }
}
pub fn is_match(w1: &str, w2: &str) -> bool {
    if w1.trim() == w2.trim() {
        true
    } else {
        false
    }
}
