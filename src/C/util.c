
#include "glad/glad.h"
#include "GLFW/glfw3.h"

 void _gladLoadGLLoader(){
   gladLoadGLLoader((GLADloadproc) glfwGetProcAddress);
 }

