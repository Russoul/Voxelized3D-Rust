#version 330 core

in vec3 colorFrag;

out vec4 fragColor;

void main()
{

	//if(pos.x >= discardRecMin.x && pos.y >= discardRecMin.y &&
	//pos.x <= discardRecMax.x && pos.y <= discardRecMax.y ) color = vec4(0,0,0,0);


	fragColor = vec4(colorFrag,1);
} 