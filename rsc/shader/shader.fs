#version 330 core

in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D uScreenTexture;

void main()
{
    // gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    // gl_FragColor = vec4(FragPosition, 1.0);
    gl_FragColor = vec4(texture(uScreenTexture, TexCoords).rgb, 1.0);
}
