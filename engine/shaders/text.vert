#version 450 core

layout (location = 0) in vec2 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 tex_coords;

uniform mat4 u_view_projection;

struct VertexOutput {
	vec4 color;
	vec2 tex_coords;
};

layout (location = 0) out VertexOutput Output;

void main() {
	Output.color = color;
	Output.tex_coords = tex_coords;

	gl_Position = u_view_projection * vec4(position, 0.0, 1.0);
}
