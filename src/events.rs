use crate::prelude::*;
use bevy::prelude::*;

pub mod prelude {
    pub use super::new;
    pub use super::NewTransition;
}

#[derive(Debug, Event)]
pub struct NewTransition {
    // angle is in degrees (0 to 360)
    pub angle: f32,
    // color is a bevy color
    pub color: Color,
    // duration is in seconds
    pub duration: f32,
    // easing
    pub ease: EaseFunction,
}

pub fn new(
    mut events: EventReader<NewTransition>,
    mut shader: Query<&mut TransitionDefinition>,
    mut state: ResMut<TransitionState>,
    time: Res<Time>,
) {
    let mut transition = shader.single_mut();
    for event in events.read() {
        let linear_color = event.color.to_linear();
        transition.color1 = transition.color2;
        transition.color2 = Vec4::new(
            linear_color.red,
            linear_color.green,
            linear_color.blue,
            linear_color.alpha,
        );
        transition.driver = 0.0;
        transition.movement_angle = event.angle;

        state.definition = event.ease;
        state.duration = event.duration;
        state.progress = 0.0;
        state.started = Some(time.elapsed_secs());
    }
}
