use std::rc::Rc;
use std::time::Instant;

use egui::ViewportId;
use glium::{Display, Surface};
use glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

use crate::app::{App, AppState};
use crate::egui_layer::Egui;
use crate::engine::fps::FpsStats;
use crate::engine::input::{set_key_state, update_input_state};
use crate::engine::layer::Layer;
use crate::math::vec2;
use crate::render::renderer::Renderer;
use crate::render::renderer2d::text::font::FontBitmap;
use crate::render::renderer2d::Renderer2d;
use crate::window::set_viewport;

pub mod component;
pub mod events;
pub mod fps;
pub mod input;
pub mod layer;

pub struct EngineContext {
  pub delta_time: f32,
  pub start_time: Instant,
  pub last_time: Instant,

  pub renderer: Renderer,
  pub renderer2d: Renderer2d,

  pub fps_stats: FpsStats,
}

impl EngineContext {
  pub fn new(renderer: Renderer, renderer2d: Renderer2d) -> Self {
    Self {
      delta_time: 0.0,
      last_time: Instant::now(),
      start_time: Instant::now(),

      renderer,
      renderer2d,

      fps_stats: FpsStats::new(),
    }
  }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  EventLoopError(EventLoopError),
}

impl From<EventLoopError> for Error {
  fn from(e: EventLoopError) -> Self {
    Self::EventLoopError(e)
  }
}

pub struct Engine {
  pub layers: Vec<Box<dyn Layer>>,
  pub egui: Option<Egui>,
  pub context: Option<EngineContext>,
  pub app: App,
}

impl Engine {
  pub fn new(window_title: &'static str, layers: Vec<Box<dyn Layer>>) -> (Self, EventLoop<()>) {
    let event_loop = EventLoop::builder().build().unwrap();

    let app = App {
      window_title,
      state: None,
    };

    let context = None;
    let egui = None;

    (
      Self {
        layers,
        egui,
        context,
        app,
      },
      event_loop,
    )
  }

  pub fn run(mut self, event_loop: EventLoop<()>) -> Result<(), Error> {
    info!("starting event loop");

    event_loop.run_app(&mut self)?;

    Ok(())
  }

  /// # Safety
  ///
  /// `self.context` must be `Some` before calling this function
  fn update(&mut self) {
    let context = self.context.as_mut().unwrap();
    let now = Instant::now();
    context.delta_time = now.duration_since(context.last_time).as_secs_f32();
    context.last_time = now;
    context.fps_stats.update((1.0 / context.delta_time) as u32);

    update_input_state();

    for layer in &mut self.layers {
      layer.update(context);
    }
  }

  /// # Safety
  ///
  /// `self.context`, `self.state` and `self.egui` must be `Some` before calling
  /// this function
  fn draw(&mut self) {
    let state = self.app.state.as_ref().unwrap();

    let mut frame = state.display.draw();

    frame.clear_color_srgb(0.0, 0.0, 0.0, 0.0);

    let context = self.context.as_mut().unwrap();

    for layer in &mut self.layers {
      layer.draw(context, &mut frame);
    }

    let egui = self.egui.as_mut().unwrap();

    egui.run(&state.window, |ctx| {
      for layer in &mut self.layers {
        layer.egui(context, ctx.clone());
      }
    });

    egui.paint(&state.display, &mut frame);

    frame.finish().unwrap();
  }

  /// # Safety
  ///
  /// `self.context`, `self.state` and `self.egui` must be `Some` before calling
  /// this function
  fn handle_window_event(&mut self, event: WindowEvent) {
    let state = self.app.state.as_ref().unwrap();

    // send events to egui first
    if self.egui.as_mut().unwrap().handle_window_event(&event, &state.window) {
      return;
    }

    match &event {
      WindowEvent::CursorMoved { position, .. } => {
        let (width, height): (f32, f32) = state.window.inner_size().into();
        let (x, y) = (position.x as f32, position.y as f32);

        let x = x / width * 2.0 - 1.0;
        let y = y / height * 2.0 - 1.0;

        input::set_mouse_position(vec2(x, y));
      }
      WindowEvent::MouseInput { state, button, .. } => {
        input::set_mouse_state(*button, *state);
      }
      WindowEvent::KeyboardInput { event, .. } => {
        set_key_state(event.physical_key.into(), event.state);
      }
      _ => (),
    }

    for i in 0..self.layers.len() {
      if self.layers[i].handle_window_event(self.context.as_mut().unwrap(), &event, &state.window) {
        break; // event was consumed
      }
    }
  }
}

impl ApplicationHandler for Engine {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    self.app.state = Some(AppState::new(event_loop, self.app.window_title, true));
    let state = self.app.state.as_ref().unwrap();
    self.context = Some(create_context(&state.display));
    self.egui = Some(Egui::new(ViewportId::ROOT, &state.display, &state.window, &event_loop));
  }

  // surely not an issue to just destroy everything
  fn suspended(&mut self, _: &ActiveEventLoop) {
    self.app.state = None;
    self.context = None;
    self.egui = None;
  }

  // Request redraw
  fn about_to_wait(&mut self, _: &ActiveEventLoop) {
    if let Some(state) = self.app.state.as_ref() {
      state.window.request_redraw();
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::RedrawRequested => {
        if self.app.state.is_some() {
          self.update();
          self.draw();
        }
      }
      WindowEvent::Resized(new_size) => {
        if let Some(state) = self.app.state.as_ref() {
          state.display.resize(new_size.into());

          set_viewport(new_size.into());

          self.handle_window_event(event)
        }
      }
      WindowEvent::CloseRequested => event_loop.exit(),
      // send every other event to engine / layers
      event => {
        if self.app.state.is_some() {
          self.handle_window_event(event)
        }
      }
    }
  }
}

fn create_context(display: &Display<WindowSurface>) -> EngineContext {
  let font = FontBitmap::from_bytes(display, include_bytes!("../../fonts/terminus/ter-u32n.bdf"));
  let font = Rc::new(font);

  let mesh_shader = shader!(display, "../../shaders/mesh.vert", "../../shaders/mesh.frag");
  let quad_shader = shader!(display, "../../shaders/quad.vert", "../../shaders/quad.frag");
  let text_shader = shader!(display, "../../shaders/text.vert", "../../shaders/text.frag");

  let renderer = Renderer::new(display, mesh_shader);
  let renderer2d = Renderer2d::new(display, quad_shader, text_shader, font);

  EngineContext::new(renderer, renderer2d)
}
