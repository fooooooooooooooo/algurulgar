#version 460 core

// Input from the vertex shader
in vec4 fragColor;

// Output color
layout(location = 0) out vec4 color;

void main() {
  // Set the output color to the interpolated vertex color
  color = fragColor;
}
