#version 460 core

// Input vertex structure
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

// Output to the fragment shader
out vec4 fragColor;

// Uniforms
uniform mat4 u_view_projection; // Premultiplied view * projection matrix
uniform mat4 u_model; // Model transformation matrix

void main() {
  // Apply the model matrix to the vertex position
  vec4 worldPosition = u_model * vec4(position, 1.0);

  // Transform the vertex position using the view-projection matrix
  gl_Position = u_view_projection * worldPosition;

  // Pass the vertex color to the fragment shader
  fragColor = color;
}
