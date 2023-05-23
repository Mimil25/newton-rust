use macroquad::prelude::*;

#[macroquad::main("Newton")]
async fn main() {

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
