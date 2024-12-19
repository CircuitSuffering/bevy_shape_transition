use crate::prelude::*;
use bevy::prelude::*;

pub mod prelude {
    pub use super::new;
    pub use super::NewTransition;
}

#[derive(Clone, Debug, Event)]
pub struct NewTransition {
    // degrees (0 to 360)
    pub angle: f32,
    // bevy color
    pub color: Color,
    pub color2: Option<Color>,
    // seconds
    pub duration: f32,
    // easing
    pub ease: EaseFunction,
}

impl Default for NewTransition {
    fn default() -> Self {
        Self {
            angle: 0.0,
            color: Color::BLACK,
            color2: Some(Color::BLACK),
            duration: 0.0,
            ease: EaseFunction::Linear,
        }
    }
}

pub fn new(
    mut new_events: EventReader<NewTransition>,
    mut shader: Query<&mut TransitionDefinition>,
    mut state: ResMut<TransitionState>,
    time: Res<Time>,
) {
    let mut transition = shader.single_mut();
    for event in new_events.read() {
        let linear_color = event.color.to_linear();

        if event.color2.is_none() {
            transition.color1 = transition.color2;
            transition.color2 = Vec4::new(
                linear_color.red,
                linear_color.green,
                linear_color.blue,
                linear_color.alpha,
            );
        } else {
            transition.color1 = Vec4::new(
                linear_color.red,
                linear_color.green,
                linear_color.blue,
                linear_color.alpha,
            );
            let linear_color2 = event.color2.unwrap().to_linear();
            transition.color2 = Vec4::new(
                linear_color2.red,
                linear_color2.green,
                linear_color2.blue,
                linear_color2.alpha,
            );
        }


        transition.driver = 0.0;
        transition.movement_angle = event.angle;

        state.definition = event.ease;
        state.duration = event.duration;
        state.progress = 0.0;
        state.started = Some(time.elapsed_secs());
    }
}
