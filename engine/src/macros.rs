#[macro_export]
macro_rules! shader {
  ($display:expr, $vert:expr, $frag:expr) => {
    $crate::render::shader::Shader::new($display, include_str!($vert), include_str!($frag))
  };
}

#[macro_export]
macro_rules! vec2 {
  ($x:expr, $y:expr) => {
    $crate::nalgebra::Vector2::new($x, $y)
  };
}

#[macro_export]
macro_rules! vec3 {
  ($x:expr, $y:expr, $z:expr) => {
    $crate::nalgebra::Vector3::new($x, $y, $z)
  };
}

#[macro_export]
macro_rules! vec4 {
  ($x:expr, $y:expr, $z:expr, $w:expr) => {
    $crate::nalgebra::Vector4::new($x, $y, $z, $w)
  };
}
