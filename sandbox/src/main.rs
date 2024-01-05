use algurulgar::engine::events::EventHandler;
use algurulgar::engine::input::{last_key_pressed, mouse_position};
use algurulgar::glium::Frame;
use algurulgar::math::Position;
use algurulgar::render::camera::ortho::OrthoCameraController;
use algurulgar::render::renderer::text::TextParams;
use algurulgar::update::UpdateHandler;
use algurulgar::winit::event::WindowEvent;
use algurulgar::winit::window::Window;
use algurulgar::{init_logger, Color, Engine, EngineContext, Layer};

#[macro_use]
extern crate algurulgar;

struct SandboxLayer {
  camera: OrthoCameraController,
  pos: Position,
}

fn main() {
  init_logger();

  info!("hello awa");

  let engine = Engine::new("awa", vec![Box::new(SandboxLayer::new())]);

  engine.run();
}

impl SandboxLayer {
  pub fn new() -> Self {
    Self {
      camera: OrthoCameraController::default(),
      pos: Position::zeros(),
    }
  }
}

impl Layer for SandboxLayer {
  fn update(&mut self, context: &mut EngineContext) {
    let time = context.start_time.elapsed().as_secs_f32();
    self.pos.x = 2.0 * (time * 4.0).sin();
    self.pos.y = 2.0 * (time * 4.0).cos();

    self.camera.update(context.delta_time);
  }

  fn draw(&mut self, context: &mut EngineContext, frame: &mut Frame) {
    // let view_projection = self.camera.camera().view_projection();

    // let _projection_uniforms = uniform! {
    //   u_view_projection: *view_projection.as_ref(),
    // };

    // context.text_renderer.draw_text(
    //   frame,
    //   &context.fps_stats.text,
    //   vec2!(-0.95, 0.7),
    //   vec2!(0.003, 0.003),
    //   Color::WHITE,
    //   view_projection,
    // );

    // let mouse = mouse_position();

    // context.text_renderer.draw_text(
    //   frame,
    //   &format!("Mouse: {:>5.2} {:>5.2}", mouse.x, mouse.y),
    //   vec2!(-0.95, 0.8),
    //   vec2!(0.003, 0.003),
    //   Color::WHITE,
    //   view_projection,
    // );

    // context.text_renderer.draw_text(
    //   frame,
    //   &format!("{:?}", last_key_pressed()),
    //   vec2!(-0.95, 0.9),
    //   vec2!(0.003, 0.003),
    //   Color::WHITE,
    //   view_projection,
    // );

    // let text = self.camera.camera().view_projection().to_string();

    // context.text_renderer.draw_text(
    //   frame,
    //   &text,
    //   vec2!(-0.95, 0.0),
    //   vec2!(0.003, 0.003),
    //   Color::WHITE,
    //   view_projection,
    // );

    let mut renderer = context.renderer.begin(&self.camera, frame);

    renderer.draw_text("awawawawa", vec2!(0.0, 0.0), &TextParams::new(context.font.clone()));

    renderer.draw_quad(self.pos, vec2!(0.5, 0.5));

    renderer.finish();
  }

  fn handle_window_event(&mut self, _context: &mut EngineContext, event: &WindowEvent, window: &Window) {
    self.camera.handle_event(event, window);
  }
}
