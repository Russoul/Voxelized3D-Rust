#version 330 core
in vec2 TexCoord;
uniform sampler2D textureID;


uniform vec3 extraColor;

out vec4 fragColor;

void main()
{
    vec4 c = texture(textureID, TexCoord);

    c.x *= extraColor.x;
    c.y *= extraColor.y;
    c.z *= extraColor.z;


    if(c.a < 0.1)
        discard;

    fragColor = c;
}
