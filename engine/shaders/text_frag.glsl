#version 140

in vec2 f_tex_coords;
in vec4 f_color;

uniform sampler2D u_texture;

out vec4 color;

void main() {
  float c = texture(u_texture, f_tex_coords).r;
  color = vec4(1, 1, 1, c) * f_color;
}
