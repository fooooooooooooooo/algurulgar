use algurulgar::engine::events::EventHandler;
use algurulgar::engine::input::{key_pressed, last_key_pressed, mouse_position};
use algurulgar::glium::Frame;
use algurulgar::math::Position;
use algurulgar::render::camera::ortho::OrthoCameraController;
use algurulgar::render::renderer::text::TextParams;
use algurulgar::update::UpdateHandler;
use algurulgar::winit::event::WindowEvent;
use algurulgar::winit::keyboard::KeyCode;
use algurulgar::winit::window::Window;
use algurulgar::{init_logger, vec2, Engine, EngineContext, Layer};
use log::info;

struct SandboxLayer {
  camera: OrthoCameraController,
  pos: Position,
  scale: f32,
}

fn main() {
  init_logger();

  info!("hello awa");

  let (engine, event_loop) = Engine::new("awa", vec![Box::new(SandboxLayer::new())]);

  engine.run(event_loop).unwrap();
}

impl SandboxLayer {
  pub fn new() -> Self {
    Self {
      camera: OrthoCameraController::default(),
      pos: Position::zeros(),
      scale: 0.05,
    }
  }
}

impl Layer for SandboxLayer {
  fn update(&mut self, context: &mut EngineContext) {
    // let time = context.start_time.elapsed().as_secs_f32();

    let mouse_position = mouse_position();
    self.pos = self.camera.camera().screen_to_world(mouse_position);

    self.camera.update(context.delta_time);

    if key_pressed(KeyCode::KeyX) {
      self.scale += 0.0001;
      println!("scale: {}", self.scale);
    } else if key_pressed(KeyCode::KeyZ) {
      self.scale -= 0.0001;
      println!("scale: {}", self.scale);
    }
  }

  fn draw(&mut self, context: &mut EngineContext, frame: &mut Frame) {
    let mouse = mouse_position();

    let text_params = TextParams::new().scale(self.scale);

    let mut renderer = context.renderer.begin(&self.camera, frame);

    renderer.draw_text(&context.fps_stats.text, vec2(-0.95, 0.7), &text_params);

    renderer.draw_text(
      &format!("Mouse: {:>5.2} {:>5.2}", mouse.x, mouse.y),
      vec2(-0.95, 0.8),
      &text_params,
    );

    renderer.draw_text(&format!("Key: {}", last_key_pressed()), vec2(-0.95, 0.6), &text_params);

    renderer.draw_text(
      &format!("Camera: {}", self.camera.camera().view_projection()),
      vec2(-0.95, 0.4),
      &text_params,
    );

    renderer.draw_text(
      &self.camera.camera().view_projection().to_string(),
      vec2(-0.95, 0.0),
      &text_params,
    );

    renderer.draw_quad(self.pos, vec2(0.5, 0.5));

    renderer.finish();
  }

  fn handle_window_event(&mut self, _context: &mut EngineContext, event: &WindowEvent, window: &Window) {
    self.camera.handle_event(event, window);
  }
}
