mod gamestate;
mod gui;
mod key_rotate;
mod packet_sender;
mod token_fetch;
pub(crate) use raylib::prelude::*;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .size(1280, 720)
        .title("JANREX 2")
        .build();

    rl.set_target_fps(120);

    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("LOADING", 10, 10, 50, Color::WHITE);
    }

    let mut gamestate = gamestate::Gamestate::new().await;

    while !rl.window_should_close() {
        gamestate.parse_network().await;
        gamestate.render(&mut rl, &thread);
    }
}
