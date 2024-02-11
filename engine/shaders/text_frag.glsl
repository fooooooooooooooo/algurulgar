#version 450 core

layout (location = 0) out vec4 o_color;

struct VertexOutput {
	vec4 color;
	vec2 tex_coords;
};

layout (location = 0) in VertexOutput Input;

layout (binding = 0) uniform sampler2D u_font_atlas;

void main() {
	vec4 texcolor = Input.color * texture(u_font_atlas, Input.tex_coords);
  o_color = vec4(texcolor.rgb, texcolor.a);
}
