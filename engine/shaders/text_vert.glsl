#version 140

in vec2 position;
in vec2 tex_coords;
in vec4 color;

uniform mat4 u_view_projection;

out vec2 f_tex_coords;
out vec4 f_color;

void main() {
  gl_Position = u_view_projection * vec4(position, 1.0, 1.0);

  f_tex_coords = tex_coords;
  f_color = color;
}
