#version 460

layout(location = 0) out vec4 color;

in vec2 v_position;
in vec4 v_color;

void main() {
  color = v_color;
}