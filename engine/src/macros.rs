#[macro_export]
macro_rules! shader {
  ($display:expr, $vert:expr, $frag:expr) => {
    $crate::render::shader::Shader::new($display, include_str!($vert), include_str!($frag))
  };
}
