use std::rc::Rc;
use std::time::Instant;

use glium::{Display, Surface};
use glutin::surface::WindowSurface;
use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::app::{App, AppState};
use crate::engine::fps::FpsStats;
use crate::engine::input::{set_key_state, update_input_state};
use crate::engine::layer::Layer;
use crate::math::vec2;
use crate::render::renderer::Renderer;
use crate::render::renderer::text::font::FontBitmap;
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

  pub fps_stats: FpsStats,
}

impl EngineContext {
  pub fn new(renderer: Renderer) -> Self {
    Self {
      delta_time: 0.0,
      last_time: Instant::now(),
      start_time: Instant::now(),

      renderer,

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

    (Self { layers, context, app }, event_loop)
  }

  pub fn run(mut self, event_loop: EventLoop<()>) -> Result<(), Error> {
    info!("starting event loop");

    event_loop.run_app(&mut self)?;

    Ok(())
  }

  fn update(context: &mut EngineContext, layers: &mut [Box<dyn Layer>]) {
    let now = Instant::now();
    context.delta_time = now.duration_since(context.last_time).as_secs_f32();
    context.last_time = now;
    context.fps_stats.update((1.0 / context.delta_time) as u32);

    update_input_state();

    for layer in layers {
      layer.update(context);
    }
  }

  fn draw(context: &mut EngineContext, layers: &mut [Box<dyn Layer>], display: &Display<WindowSurface>) {
    let mut frame = display.draw();

    frame.clear_color_srgb(0.0, 0.0, 0.0, 0.0);

    for layer in layers {
      layer.draw(context, &mut frame);
    }

    frame.finish().unwrap();
  }

  fn handle_window_event(
    context: &mut EngineContext,
    layers: &mut [Box<dyn Layer>],
    event: &WindowEvent,
    window: &Window,
  ) {
    match event {
      WindowEvent::CursorMoved { position, .. } => {
        let (width, height): (f32, f32) = window.inner_size().into();
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

    for layer in layers {
      layer.handle_window_event(context, event, window);
    }
  }
}

impl ApplicationHandler for Engine {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let state = AppState::new(event_loop, self.app.window_title, true);
    self.context = Some(create_context(&state.display));
    self.app.state = Some(state);
  }

  fn suspended(&mut self, _: &ActiveEventLoop) {
    self.app.state = None
  }

  // Request redraw
  fn about_to_wait(&mut self, _: &ActiveEventLoop) {
    if let Some(state) = &self.app.state {
      state.window.request_redraw();
    }
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::RedrawRequested => {
        if let Some(state) = &mut self.app.state {
          let context = self.context.as_mut().unwrap();
          Self::update(context, &mut self.layers);
          Self::draw(context, &mut self.layers, &state.display);
        }
      }
      WindowEvent::Resized(new_size) => {
        if let Some(state) = &mut self.app.state {
          state.display.resize(new_size.into());

          set_viewport(new_size.into());

          Self::handle_window_event(self.context.as_mut().unwrap(), &mut self.layers, &event, &state.window);
        }
      }
      WindowEvent::CloseRequested => event_loop.exit(),
      // send every other event to engine / layers
      event => {
        if let Some(state) = &mut self.app.state {
          Self::handle_window_event(self.context.as_mut().unwrap(), &mut self.layers, &event, &state.window);
        }
      }
    }
  }
}

fn create_context(display: &Display<WindowSurface>) -> EngineContext {
  let font = FontBitmap::from_bytes(display, include_bytes!("../../fonts/terminus/ter-u32n.bdf"));
  let font = Rc::new(font);

  let quad_shader = shader!(display, "../../shaders/vert.glsl", "../../shaders/frag.glsl");
  let text_shader = shader!(display, "../../shaders/text_vert.glsl", "../../shaders/text_frag.glsl");

  let renderer = Renderer::new(display, quad_shader, text_shader, font);

  EngineContext::new(renderer)
}
