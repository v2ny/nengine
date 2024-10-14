#version 330 core

out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture1;
uniform bool has_alpha;

void main() {
    vec4 texColor = texture(texture1, TexCoord);

    // If texture has alpha, handle transparency.
    if (has_alpha) {
        if (texColor.a < 0.1)
            discard;  // Skip fully transparent pixels
        
        // Apply the blending color (multiplying the texture color by blending color)
        FragColor = texColor;
    } else {
        // Apply blending color to fully opaque textures
        FragColor = texColor;  // Set alpha to 1.0 for opaque textures
    }
}
