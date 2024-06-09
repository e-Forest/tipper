use macroquad::prelude::*;

use tipper::*;

#[macroquad::main["Tipper"]]
async fn main() {
    let mut data = Data::new();
    data.add_words(20);
    loop {
        clear_background(DARKGRAY);
        data.draw_words();
        data.draw_load();
        data.update_load();
        next_frame().await;
        if is_key_down(KeyCode::Escape) {
            break;
        }
    }
}
