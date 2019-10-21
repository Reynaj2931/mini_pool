use ggez::*;

// defining the state of the game
pub struct State {
    pub dt: std::time::Duration,

    pub hole: CirclePhys,
    pub white_ball: CirclePhys,
    pub red_ball: CirclePhys,
}

// defining the circle, graphics and physics
// P.S. Physics coming later
pub struct CirclePhys {
    radius: f32,
    x: f32,
    y: f32,
    color: graphics::Color,
}

// implementing circle, constructor and simple movement
impl CirclePhys {
    pub fn new(radius: f32, x: f32, y: f32, color: graphics::Color) -> CirclePhys {
        CirclePhys {
            radius: radius,
            x: x,
            y: y,
            color: color,
        }
    }
    
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

// implementing EventHandler trait for the State, i.e. implementing the update and draw methods
impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        let mouse_position = ggez::input::mouse::position(ctx);

        self.white_ball.move_to(mouse_position.x, mouse_position.y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.1, 1.0].into());

        let hole_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ggez::nalgebra::Point2::new(0.0, 0.0),
                self.hole.radius,
                0.1,
                self.hole.color,
            ).unwrap();
        graphics::draw(ctx, &hole_mesh, (ggez::nalgebra::Point2::new(self.hole.x, self.hole.y),))?;


        let white_ball_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ggez::nalgebra::Point2::new(0.0, 0.0),
                self.white_ball.radius,
                0.1,
                self.white_ball.color,
            ).unwrap();

        let red_ball_mesh = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ggez::nalgebra::Point2::new(0.0, 0.0),
                self.red_ball.radius,
                0.1,
                self.red_ball.color,
            ).unwrap();

        graphics::draw(ctx, &white_ball_mesh, (ggez::nalgebra::Point2::new(self.white_ball.x, self.white_ball.y),))?;
        graphics::draw(ctx, &red_ball_mesh, (ggez::nalgebra::Point2::new(self.red_ball.x, self.red_ball.y),))?;
        graphics::present(ctx)?;
        Ok(())
    }
}
