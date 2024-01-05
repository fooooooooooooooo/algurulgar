use std::num::NonZeroU32;

use glium::Display;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

use crate::window::set_viewport;
use crate::VSYNC;

pub struct AppState {
  pub display: Display<WindowSurface>,
  pub window: Window,
}

impl AppState {
  pub fn new(event_loop: &EventLoopWindowTarget<()>, window_title: &'static str, visible: bool) -> AppState {
    let window_builder = WindowBuilder::new().with_title(window_title).with_visible(visible);
    let config_template_builder = ConfigTemplateBuilder::new().with_multisampling(4);
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
      .build(event_loop, config_template_builder, |mut configs| {
        configs.next().unwrap()
      })
      .unwrap();
    let window = window.unwrap();

    let raw_window_handle = window.raw_window_handle();
    let context_attributes = ContextAttributesBuilder::new()
      .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 0))))
      .build(Some(raw_window_handle));

    let not_current_gl_context = unsafe {
      gl_config
        .display()
        .create_context(&gl_config, &context_attributes)
        .unwrap()
    };

    // Determine our framebuffer size based on the window size, or default to
    // 800x600 if it's invisible
    let (width, height): (u32, u32) = if visible {
      window.inner_size().into()
    } else {
      (800, 600)
    };

    // set engine viewport size used for camera and stuff
    set_viewport((width, height));

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
      raw_window_handle,
      NonZeroU32::new(width).unwrap(),
      NonZeroU32::new(height).unwrap(),
    );
    // Now we can create our surface, use it to make our context current and finally
    // create our display
    let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };
    let current_context = not_current_gl_context.make_current(&surface).unwrap();

    let swap_interval = if VSYNC {
      SwapInterval::Wait(NonZeroU32::new(1).unwrap())
    } else {
      SwapInterval::DontWait
    };

    surface.set_swap_interval(&current_context, swap_interval).unwrap();
    let display = Display::from_context_surface(current_context, surface).unwrap();

    AppState { display, window }
  }
}

pub struct App {
  pub window_title: &'static str,
  pub event_loop: EventLoop<()>,
  pub state: Option<AppState>,
}
