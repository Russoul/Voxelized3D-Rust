#version 330 core
in vec2 TexCoord;
in vec3 Color;
uniform sampler2D textureID;


void main()
{
    vec4 c = texture2D(textureID, TexCoord);

    //c *= vec4(Color, 1);

    /*if(c.a < 0.1)
        discard;*/

    gl_FragColor = c;
    //gl_FragColor = vec4(0,0,0,1);
}
