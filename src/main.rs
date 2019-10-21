use ggez::*;
use mini_pool::State;
use mini_pool::CirclePhys;

fn main() {
    let ball_radius = 25.0;
    let hole_margine = 10.0;

    // Initializing the game state
    let state = &mut State {
        dt: std::time::Duration::new(0, 0),

        hole: CirclePhys::new(ball_radius, hole_margine, hole_margine, graphics::BLACK),

        white_ball: CirclePhys::new(ball_radius, 400.0, 300.0, graphics::WHITE),
        red_ball: CirclePhys::new(ball_radius, 600.0, 150.0, [0.55, 0.2, 0.2, 1.0].into()),
    };

    // preparing the context, the loop and the event handler
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "mezanix")
        .conf(c)
        .build()
        .unwrap();

    // running the game loop
    event::run(ctx, event_loop, state).unwrap();

    // ending the game
    println!("End of the game.");
}
