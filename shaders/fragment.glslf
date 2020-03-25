#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform Shadow {
    float u_Rate;
};


void main() {
    float value = 1.0 - texture(t_Texture, v_Uv).a;
    float transparency = texture(t_Texture, v_Uv).a * u_Rate;
    Target0 = vec4(value, value, value, transparency);
}