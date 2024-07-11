use std::sync::atomic::{AtomicU32, Ordering};

use atomic_float::AtomicF32;

pub type Viewport = (u32, u32);
pub type ViewportF = (f32, f32);

static VIEWPORT: (AtomicU32, AtomicU32) = (AtomicU32::new(0), AtomicU32::new(0));
static VIEWPORT_F: (AtomicF32, AtomicF32) = (AtomicF32::new(0.0), AtomicF32::new(0.0));
static ASPECT_RATIO: AtomicF32 = AtomicF32::new(1.0);

#[inline]
pub fn viewport() -> Viewport {
  (VIEWPORT.0.load(Ordering::Acquire), VIEWPORT.1.load(Ordering::Acquire))
}

#[inline]
pub fn viewport_f() -> ViewportF {
  (
    VIEWPORT_F.0.load(Ordering::Acquire),
    VIEWPORT_F.1.load(Ordering::Acquire),
  )
}

#[inline]
pub fn set_viewport((width, height): Viewport) {
  VIEWPORT.0.store(width, Ordering::Release);
  VIEWPORT.1.store(height, Ordering::Release);

  VIEWPORT_F.0.store(width as f32, Ordering::Release);
  VIEWPORT_F.1.store(height as f32, Ordering::Release);

  ASPECT_RATIO.store(width as f32 / height as f32, Ordering::Release);
}

#[inline]
pub fn aspect_ratio() -> f32 {
  ASPECT_RATIO.load(Ordering::Acquire)
}
