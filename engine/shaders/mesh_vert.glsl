#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

out vec3 v_position;
out vec4 v_color;

uniform mat4 u_view_projection;
uniform mat4 u_transform;

void main() {
  v_position = position;
  v_color = color;
  gl_Position = u_view_projection * vec4(position, 1.0) * u_transform;
}
