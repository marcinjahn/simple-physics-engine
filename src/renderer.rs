use crate::experiment::Experiment;
use macroquad::color::{Color, BLACK, GRAY, WHITE};
use macroquad::prelude::{clear_background, draw_circle};
use macroquad::text::draw_text;

pub struct Renderer<'a> {
    pub experiment: &'a Experiment,
    pub render_ball_ids: bool,
}

impl Renderer<'_> {
    pub fn render(&self) {
        clear_background(Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        });

        self.experiment
            .constraint
            .as_ref()
            .map(|constraint| constraint.render());

        self.experiment.balls.iter().for_each(|ball| {
            draw_circle(
                ball.verlet_object.position_current.x,
                ball.verlet_object.position_current.y,
                ball.characteristics.radius,
                ball.characteristics.color,
            );

            if !self.render_ball_ids {
                return;
            }

            let id_x = ball.verlet_object.position_current.x - ball.characteristics.radius / 4.0;
            let id_y = ball.verlet_object.position_current.y + ball.characteristics.radius / 4.0;
            draw_text(
                ball.characteristics.id.to_string().as_str(),
                id_x,
                id_y,
                ball.characteristics.radius,
                WHITE,
            )
        });
    }
}
