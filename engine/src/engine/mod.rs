use std::rc::Rc;
use std::time::Instant;

use glium::{Display, Surface};
use glutin::surface::WindowSurface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::window::Window;

use crate::app::{App, AppState};
use crate::engine::fps::FpsStats;
use crate::engine::input::{set_key_state, update_input_state};
use crate::engine::layer::Layer;
use crate::render::renderer::Renderer;
use crate::render::text::batch::TextRenderer;
use crate::render::text::font::FontBitmap;
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
  pub text_renderer: TextRenderer,

  pub fps_stats: FpsStats,
  pub font: Rc<FontBitmap>,
}

impl EngineContext {
  pub fn new(renderer: Renderer, text_renderer: TextRenderer, font: Rc<FontBitmap>) -> Self {
    Self {
      delta_time: 0.0,
      last_time: Instant::now(),
      start_time: Instant::now(),

      renderer,
      text_renderer,

      fps_stats: FpsStats::new(),

      font,
    }
  }
}

pub struct Engine {
  pub app: App,

  pub layers: Vec<Box<dyn Layer>>,
  pub context: Option<EngineContext>,
}

impl Engine {
  pub fn new(window_title: &'static str, layers: Vec<Box<dyn Layer>>) -> Self {
    let event_loop = EventLoopBuilder::new().build();

    let app = App {
      window_title,
      event_loop,
      state: None,
    };

    let context = None;

    Self { app, layers, context }
  }

  pub fn run(self) -> ! {
    let Self {
      mut app,
      mut layers,
      mut context,
    } = self;

    info!("starting event loop");

    app.event_loop.run(move |event, window_target, control_flow| {
      match event {
        Event::Resumed => {
          let state = AppState::new(window_target, app.window_title, true);
          context = Some(create_context(&state.display));
          app.state = Some(state);
        }
        Event::Suspended => app.state = None,
        Event::RedrawRequested(_) => {
          if let Some(state) = &mut app.state {
            let context = context.as_mut().unwrap();
            Self::update(context, &mut layers);
            Self::draw(context, &mut layers, &state.display);
          }
        }
        // By requesting a redraw in response to a RedrawEventsCleared event we get continuous rendering.
        // For applications that only change due to user input you could remove this handler.
        Event::RedrawEventsCleared => {
          if let Some(state) = &app.state {
            state.window.request_redraw();
          }
        }
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::Resized(new_size) => {
            if let Some(state) = &mut app.state {
              state.display.resize(new_size.into());

              set_viewport(new_size.into());

              Self::handle_window_event(context.as_mut().unwrap(), &mut layers, &event, &state.window);
            }
          }
          WindowEvent::CloseRequested => control_flow.set_exit(),
          // Every other event
          event => {
            if let Some(state) = &mut app.state {
              Self::handle_window_event(context.as_mut().unwrap(), &mut layers, &event, &state.window);
            }
          }
        },
        _ => (),
      };
    });
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

        input::set_mouse_position(vec2!(x, y));
      }
      WindowEvent::MouseInput { state, button, .. } => {
        input::set_mouse_state(*button, *state);
      }
      WindowEvent::KeyboardInput { input, .. } => {
        if let Some(key) = input.virtual_keycode {
          set_key_state(key, input.state);
        }
      }
      _ => (),
    }

    for layer in layers {
      layer.handle_window_event(context, event, window);
    }
  }
}

fn create_context(display: &Display<WindowSurface>) -> EngineContext {
  let font = FontBitmap::from_bytes(display, include_bytes!("../../fonts/terminus/ter-u32n.bdf"));
  let font = Rc::new(font);

  let text_renderer = TextRenderer::new(font.clone(), display);

  let shader = shader!(display, "../../shaders/vert.glsl", "../../shaders/frag.glsl");

  let mut renderer = Renderer::new(display);

  renderer.add_shader(shader);

  EngineContext::new(renderer, text_renderer, font)
}
