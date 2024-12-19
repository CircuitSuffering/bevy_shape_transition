pub mod events;
pub mod shader;

use crate::prelude::*;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
    },
    math::Curve,
    prelude::*,
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
    window::WindowResized,
};

pub mod prelude {
    pub use crate::events::prelude::*;
    pub use crate::shader::prelude::*;
    pub use crate::TransitionPlugin;
    pub use crate::TransitionState;
}

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TRANSITION_SHADER_HANDLE,
            "..\\assets\\shader.wgsl",
            Shader::from_wgsl
        );
        app.register_type::<TransitionDefinition>();
        app.add_plugins((
            ExtractComponentPlugin::<TransitionDefinition>::default(),
            UniformComponentPlugin::<TransitionDefinition>::default(),
        ));

        app.add_systems(Startup, startup);
        app.add_systems(PreUpdate, (new, on_resize));
        app.add_systems(Update, update);
        app.init_resource::<TransitionState>();
        app.add_event::<NewTransition>();

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<TransitionNode>>(Core3d, TransitionLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    TransitionLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<TransitionNode>>(Core2d, TransitionLabel)
            .add_render_graph_edges(
                Core2d,
                (Node2d::EndMainPass, TransitionLabel, Node2d::Tonemapping),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<TransitionPipeline>();
    }
}

#[derive(Debug, Resource)]
pub struct TransitionState {
    pub definition: EaseFunction,
    // duration is in seconds
    pub duration: f32,
    // progress is a value between 0 and 1
    pub progress: f32,
    // started is the time in seconds when the transition started
    pub started: Option<f32>,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self {
            definition: EaseFunction::Linear,
            duration: 0.0,
            progress: 1.0,
            started: None,
        }
    }
}

fn startup(
    mut commands: Commands
) {
    commands.spawn(TransitionDefinition::default());
}

fn update(
    mut shader_query: Query<&mut TransitionDefinition>,
    mut state: ResMut<TransitionState>,
    time: Res<Time>,
) {
    // Early return if transition hasn't started
    let started = match state.started {
        Some(f) => f,
        None => return,
    };

    // Calculate elapsed time
    let now = time.elapsed_secs() - started;

    let buffer = 0.01;
    // Reset logic if the transition duration is exceeded
    if now > state.duration + buffer {
        if let Ok(mut shader) = shader_query.get_single_mut() {
            state.started = None;
            state.progress = 0.0;
            shader.driver = 0.0;
            shader.color1 = shader.color2;
            shader.color2 = shader.color2;
        }
        return;
    }

    // Update shader and state properties
    if let Ok(mut shader) = shader_query.get_single_mut() {
        // Update progress and clamp to 1.0
        state.progress = state.progress + time.delta_secs() / state.duration;

        // Sample the easing curve
        if let Some(eased_value) = EasingCurve::new(0.0, 1.0, state.definition).sample(state.progress) {
            // Directly assign eased value to driver
            shader.driver = eased_value;
        }
    }
}

fn on_resize(
    mut current_definition: Query<&mut TransitionDefinition>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for event in resize_reader.read() {
        current_definition.single_mut().resolution =
            Vec2::new(event.width as f32, event.height as f32);

        println!("Resolution: {:?}", current_definition.single().resolution);
    }
}
