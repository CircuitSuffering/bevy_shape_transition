use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, ExtractComponent},
        globals::{GlobalsBuffer, GlobalsUniform},
        render_graph::{NodeRunError, RenderGraphContext, RenderLabel, ViewNode},
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
    },
};

pub mod prelude {
    pub use super::TransitionDefinition;
    pub use super::TransitionLabel;
    pub use super::TransitionNode;
    pub use super::TransitionPipeline;
    pub use super::TRANSITION_SHADER_HANDLE;
}

pub const TRANSITION_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xc5e6dc9d9505418b924cf2c222bdf086);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct TransitionLabel;

#[derive(Default)]
pub struct TransitionNode;

impl ViewNode for TransitionNode {
    type ViewQuery = &'static ViewTarget;
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let video_post_pipeline = world.resource::<TransitionPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(video_post_pipeline.pipeline_id)
        else {
            return Ok(());
        };
        let settings_uniforms = world.resource::<ComponentUniforms<TransitionDefinition>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };
        let globals_buffer = world.resource::<GlobalsBuffer>();
        let Some(global_uniforms) = globals_buffer.buffer.binding() else {
            return Ok(());
        };
        let post_process = view_target.post_process_write();
        let bind_group = render_context.render_device().create_bind_group(
            "transition_bind_group",
            &video_post_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &video_post_pipeline.sampler,
                settings_binding.clone(),
                global_uniforms,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("transition_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

// This contains global data used by the render pipeline. This will be created once on startup.
#[derive(Resource)]
pub struct TransitionPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for TransitionPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "transition_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<TransitionDefinition>(false),
                    uniform_buffer::<GlobalsUniform>(false),
                ),
            ),
        );
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = TRANSITION_SHADER_HANDLE.clone();

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("transition_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

// This is the component that will get passed to the shader
#[derive(Debug, Component, Reflect, Clone, Copy, ExtractComponent, ShaderType)]
#[reflect(Debug, Component, Default)]
pub struct TransitionDefinition {
    pub color1: Vec4,
    pub color2: Vec4,
    pub resolution: Vec2,
    pub driver: f32,
    pub movement_angle: f32,
    #[cfg(feature = "webgl2")]
    webgl2_padding: Vec2,
}

impl Default for TransitionDefinition {
    fn default() -> Self {
        Self {
            color1: Vec4::new(0.0, 0.0, 0.0, 0.0),
            color2: Vec4::new(0.0, 0.0, 0.0, 0.0),
            resolution: Vec2::ZERO,
            driver: 0.0,
            movement_angle: 0.0,
            #[cfg(feature = "webgl2")]
            webgl2_padding: Vec2::ZERO,
        }
    }
}
