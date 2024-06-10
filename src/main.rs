use macroquad::prelude::*;

use tipper::*;

#[macroquad::main["Tipper"]]
async fn main() {
    let mut data = Data::new();
    data.add_words(5, screen_width() * 0.8);
    set_fullscreen(true);
    loop {
        clear_background(DARKGRAY);
        if data.ship.radius <= 0. {
            draw_text(
                "Game Over",
                data.world_rect.w / 2.,
                data.world_rect.h * 2.,
                40.,
                PINK,
            );
        } else {
            data.update();
        }
        next_frame().await;
        if is_key_down(KeyCode::Escape) {
            break;
        }
    }
}
