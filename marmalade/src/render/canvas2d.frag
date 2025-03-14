#version 300 es
            
precision highp float;

in vec4 vColor;
in vec2 vTexcoord;

uniform sampler2D uTexture;

out vec4 outColor;

void main() {
    outColor = texture(uTexture, vTexcoord) * vColor;
    outColor.rgb *= outColor.a;
}
