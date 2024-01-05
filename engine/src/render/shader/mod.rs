use glium::draw_parameters::{
  ClipControlDepth, ClipControlOrigin, DepthClamp, PolygonOffset, ProvokingVertex, Stencil,
};
use glium::{
  BackfaceCullingMode, Blend, BlendingFunction, Depth, DepthTest, Display, DrawParameters, LinearBlendingFactor,
  PolygonMode, Program, StencilOperation, StencilTest,
};
use glutin::surface::WindowSurface;

pub struct Shader {
  program: Program,
}

impl Shader {
  pub fn new(display: &Display<WindowSurface>, vertex_shader: &str, fragment_shader: &str) -> Self {
    let program = Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

    Self { program }
  }

  #[inline]
  pub fn program(&self) -> &Program {
    &self.program
  }
}

// make Default::default() const holy fuck
pub const DRAW_PARAMETERS: DrawParameters<'static> = DrawParameters {
  blend: Blend {
    color: BlendingFunction::Addition {
      source: LinearBlendingFactor::SourceAlpha,
      destination: LinearBlendingFactor::OneMinusSourceAlpha,
    },
    alpha: BlendingFunction::Addition {
      source: LinearBlendingFactor::SourceAlpha,
      destination: LinearBlendingFactor::OneMinusSourceAlpha,
    },
    constant_value: (0.0, 0.0, 0.0, 0.0),
  },
  depth: Depth {
    test: DepthTest::Overwrite,
    write: false,
    range: (0.0, 1.0),
    clamp: DepthClamp::NoClamp,
  },
  stencil: Stencil {
    test_clockwise: StencilTest::AlwaysPass,
    reference_value_clockwise: 0,
    write_mask_clockwise: 0xFFFFFFFF,
    fail_operation_clockwise: StencilOperation::Keep,
    pass_depth_fail_operation_clockwise: StencilOperation::Keep,
    depth_pass_operation_clockwise: StencilOperation::Keep,
    test_counter_clockwise: StencilTest::AlwaysPass,
    reference_value_counter_clockwise: 0,
    write_mask_counter_clockwise: 0xFFFFFFFF,
    fail_operation_counter_clockwise: StencilOperation::Keep,
    pass_depth_fail_operation_counter_clockwise: StencilOperation::Keep,
    depth_pass_operation_counter_clockwise: StencilOperation::Keep,
  },
  color_mask: (true, true, true, true),
  line_width: None,
  point_size: None,
  backface_culling: BackfaceCullingMode::CullingDisabled,
  polygon_mode: PolygonMode::Fill,
  clip_planes_bitmask: 0,
  multisampling: true,
  dithering: true,
  viewport: None,
  scissor: None,
  draw_primitives: true,
  samples_passed_query: None,
  time_elapsed_query: None,
  primitives_generated_query: None,
  transform_feedback_primitives_written_query: None,
  condition: None,
  transform_feedback: None,
  smooth: None,
  provoking_vertex: ProvokingVertex::LastVertex,
  primitive_bounding_box: (-1.0..1.0, -1.0..1.0, -1.0..1.0, -1.0..1.0),
  primitive_restart_index: false,
  polygon_offset: PolygonOffset {
    factor: 0.0,
    units: 0.0,
    point: false,
    line: false,
    fill: false,
  },
  clip_control_origin: ClipControlOrigin::LowerLeft,
  clip_control_depth: ClipControlDepth::NegativeOneToOne,
};
