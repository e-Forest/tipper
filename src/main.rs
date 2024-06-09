use macroquad::prelude::*;

use tipper::*;

#[macroquad::main["Tipper"]]
async fn main() {
    let mut data = Data::new();
    data.add_words(5, screen_width() * 0.8);
    loop {
        clear_background(DARKGRAY);
        data.update();
        next_frame().await;
        if is_key_down(KeyCode::Escape) {
            break;
        }
    }
}
