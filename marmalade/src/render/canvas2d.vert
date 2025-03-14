#version 300 es
        
in vec2 aPosition;
in vec2 aTexcoord;
in vec4 aColor;

uniform mat3 uViewMatrix;

out vec4 vColor;
out vec2 vTexcoord;

void main() {
    gl_Position = vec4((uViewMatrix * vec3(aPosition, 1.)).xy, 1., 1.);

    vColor = aColor;
    vTexcoord = aTexcoord;
}
