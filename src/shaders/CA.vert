#version 300 es
in vec4 position;
in uint cellTypeVert;

flat out uint cellTypeFrag;

void main() {
  cellTypeFrag = cellTypeVert;
  gl_Position = position;
}
