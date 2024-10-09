#version 410 core
  
out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture1;
uniform bool has_alpha;

void main() {
    vec4 texColor = texture(texture1, TexCoord);

	if (has_alpha) {
        if (texColor.a < 0.1)
            discard;
        FragColor = vec4(texColor.rgb, texColor.a);
    } else {
        FragColor = vec4(texColor.rgb, 1.0); // Apply texture without alpha handling
    }
}