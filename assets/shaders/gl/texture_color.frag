#version 330 core
in vec2 TexCoord;
in vec3 Color;
uniform sampler2D textureID;

out vec4 fragColor;

void main()
{
    vec4 c = texture(textureID, TexCoord);

    //c *= vec4(Color, 1);

    /*if(c.a < 0.1)
        discard;*/

    fragColor = c;
    //gl_FragColor = vec4(0,0,0,1);
}
