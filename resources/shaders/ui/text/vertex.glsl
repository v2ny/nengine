#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 texCoord;

uniform mat4 projection;
uniform mat4 model;

out vec2 TexCoord;

void main() {
	gl_Position = projection * model * vec4(aPos, 1.0);
	TexCoord = texCoord;
}