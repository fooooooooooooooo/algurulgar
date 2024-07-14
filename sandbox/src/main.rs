use algurulgar::debug::widgets::{MatrixWidget, VectorWidget};
use algurulgar::egui::{Color32, Context, Widget};
use algurulgar::engine::events::EventHandler;
use algurulgar::engine::input::{key_pressed, last_key_pressed, mouse_position};
use algurulgar::glium::Frame;
use algurulgar::math::Position;
use algurulgar::nalgebra::Matrix4;
use algurulgar::render::camera::ortho::OrthoCameraController;
use algurulgar::render::renderer::mesh::Mesh;
use algurulgar::render::renderer2d::text::TextParams;
use algurulgar::update::UpdateHandler;
use algurulgar::winit::event::WindowEvent;
use algurulgar::winit::keyboard::KeyCode;
use algurulgar::winit::window::Window;
use algurulgar::{egui, init_logger, vec2, vec3, Engine, EngineContext, Layer, Vec3};
use log::info;

struct SandboxLayer {
  camera: OrthoCameraController,
  pos: Position,
  scale: f32,

  bunny: Mesh,
  bunny_pos: Vec3,
  bunny_rot: Vec3,
  bunny_scale: Vec3,

  bunny_trans: Matrix4<f32>,

  bunny_debug: String,
}

fn main() {
  init_logger();

  info!("hello awa");

  let (engine, event_loop) = Engine::new("awa", vec![Box::new(SandboxLayer::new())]);

  engine.run(event_loop).unwrap();
}

impl SandboxLayer {
  pub fn new() -> Self {
    let bunny = Mesh::load_obj(include_str!("../../assets/bunny.obj"));

    Self {
      camera: OrthoCameraController::default(),
      pos: Position::zeros(),
      scale: 0.05,
      bunny,
      bunny_pos: Vec3::zeros(),
      bunny_rot: Vec3::zeros(),
      bunny_scale: vec3(2.0, 2.0, 2.0),
      bunny_trans: Matrix4::identity(),
      bunny_debug: String::new(),
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

    // move bunny
    if key_pressed(KeyCode::ArrowUp) {
      self.bunny_pos.y += 0.01;
    } else if key_pressed(KeyCode::ArrowDown) {
      self.bunny_pos.y -= 0.01;
    }

    if key_pressed(KeyCode::ArrowLeft) {
      self.bunny_pos.x -= 0.01;
    } else if key_pressed(KeyCode::ArrowRight) {
      self.bunny_pos.x += 0.01;
    }

    if key_pressed(KeyCode::BracketLeft) {
      self.bunny_pos.z += 0.01;
    } else if key_pressed(KeyCode::BracketRight) {
      self.bunny_pos.z -= 0.01;
    }

    // rotate bunny
    if key_pressed(KeyCode::KeyI) {
      self.bunny_rot.x += 0.01;
    } else if key_pressed(KeyCode::KeyK) {
      self.bunny_rot.x -= 0.01;
    }

    if key_pressed(KeyCode::KeyJ) {
      self.bunny_rot.y += 0.01;
    } else if key_pressed(KeyCode::KeyL) {
      self.bunny_rot.y -= 0.01;
    }

    if key_pressed(KeyCode::KeyU) {
      self.bunny_rot.z += 0.01;
    } else if key_pressed(KeyCode::KeyO) {
      self.bunny_rot.z -= 0.01;
    }

    // scale bunny
    if key_pressed(KeyCode::KeyN) {
      self.bunny_scale.x += 0.01;
      self.bunny_scale.y += 0.01;
      self.bunny_scale.z += 0.01;
    } else if key_pressed(KeyCode::KeyM) {
      self.bunny_scale.x -= 0.01;
      self.bunny_scale.y -= 0.01;
      self.bunny_scale.z -= 0.01;
    }

    self.bunny_trans = Matrix4::new_rotation(self.bunny_rot);
    self.bunny_trans.append_translation_mut(&self.bunny_pos);
    self.bunny_trans.append_nonuniform_scaling_mut(&self.bunny_scale);

    self.bunny_debug = format!(
      "pos: {:?}\nrot: {:?}\nscale: {:?}\ntransform: {}",
      self.bunny_pos, self.bunny_rot, self.bunny_scale, self.bunny_trans
    );
  }

  fn draw(&mut self, context: &mut EngineContext, frame: &mut Frame) {
    let mouse = mouse_position();

    let text_params = TextParams::new().scale(self.scale);

    let mut renderer = context.renderer.begin(&self.camera, frame);

    renderer.draw_transform(&self.bunny, &self.bunny_trans);

    renderer.finish();

    let mut renderer2d = context.renderer2d.begin(&self.camera, frame);

    renderer2d.draw_text(&context.fps_stats.text, vec2(-0.95, 0.7), &text_params);

    renderer2d.draw_text(
      &format!("Mouse: {:>5.2} {:>5.2}", mouse.x, mouse.y),
      vec2(-0.95, 0.8),
      &text_params,
    );

    renderer2d.draw_text(&format!("Key: {}", last_key_pressed()), vec2(-0.95, 0.6), &text_params);

    renderer2d.draw_text(
      &format!("Camera: {}", self.camera.camera().view_projection()),
      vec2(-0.95, 0.4),
      &text_params,
    );

    // display bunny pos, rot, scale, and then the transform
    renderer2d.draw_text(&self.bunny_debug, vec2(-0.95, -0.8), &text_params);

    renderer2d.draw_quad(self.pos, vec2(0.5, 0.5));

    renderer2d.finish();
  }

  fn egui(&mut self, context: &mut EngineContext, ctx: Context) {
    egui::TopBottomPanel::top("top_panel").show(&ctx, |ui| {
      // fps
      ui.monospace(context.fps_stats.text.clone());

      ui.label(format!("Mouse: {:?}", mouse_position()));
      ui.label(format!("Key: {:?}", last_key_pressed()));

      ui.label("Camera:");
      MatrixWidget::new(self.camera.camera().view_projection()).ui(ui);

      // bunny
      ui.horizontal(|ui| {
        ui.label("Bunny Pos:");
        VectorWidget::new(&self.bunny_pos).ui(ui);
      });

      ui.horizontal(|ui| {
        ui.label("Bunny Rot:");
        VectorWidget::new(&self.bunny_rot).ui(ui);
      });

      ui.horizontal(|ui| {
        ui.label("Bunny Scale:");
        VectorWidget::new(&self.bunny_scale).ui(ui);
      });

      ui.label("Bunny Trans:");
      MatrixWidget::new(&self.bunny_trans).ui(ui);
    });

    egui::Window::new("hello awa").show(&ctx, |ui| {
      ui.label("hello awa");
      ui.colored_label(Color32::LIGHT_GREEN, "hello awa");
    });
  }

  fn handle_window_event(&mut self, _context: &mut EngineContext, event: &WindowEvent, window: &Window) -> bool {
    self.camera.handle_event(event, window);

    false
  }
}
