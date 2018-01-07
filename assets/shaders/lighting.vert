#version 330 core
  
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec3 normal;

out vec3 colorFrag;
out vec3 normalFrag;
out vec3 posFrag;

uniform mat4 P;
uniform mat4 V;

void main()
{
	colorFrag = color;
	normalFrag = normal;
	posFrag = position;
	vec4 p = vec4(position.x, position.y, position.z, 1.0);
    gl_Position = P * V * p;
}
