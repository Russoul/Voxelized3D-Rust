#version 330 core

in vec3 posFrag;
in vec3 colorFrag;
in vec3 normalFrag;


struct PointLight
{
        vec3 pos;
        vec3 color;
};

uniform PointLight pointLight;

void main()
{
    vec3 ambientLight = vec3(0.2,0.2,0.2);


    vec3 toPointLight = pointLight.pos - posFrag;

    float dist = length(toPointLight);
    vec3 dir = toPointLight / dist;

    vec3 diffuse = pointLight.color * (max(dot(dir,normalFrag), 0.0) / dist / dist);


    vec3 resulting = colorFrag * (ambientLight + diffuse);



	gl_FragColor = vec4(resulting,1);
} 