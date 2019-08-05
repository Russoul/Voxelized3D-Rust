
#include <stdlib.h>
#include <stdio.h>
#include "glad/glad.h"
#include "GLFW/glfw3.h"
#include "util.h"
#include <string.h>

#include <dirent.h> //POSIX only, used to play with files






//returns all found filenames
//filenames.names must be freed (both pointer levels)
filenames find_all_files_in_dir(char *path) 
{
  DIR           *d;
  struct dirent *dir;
  d = opendir(path);
  if (d)
  {

    int file_count = 0;

    while ((dir = readdir(d)) != NULL) {//count the amount of files in directory
      if (dir->d_type == DT_REG) { /* If the entry is a regular file */
	     file_count++;
      }
    }

    char **names = malloc(sizeof (char*) * file_count);

    closedir(d);
    d = opendir(path); //reopen directory

    int i = 0;
    while ((dir = readdir(d)) != NULL)
    {
      if(dir->d_type == DT_REG) {
	
        int size = strlen(dir->d_name);   //
        char* copy = malloc(size);     //
        memcpy(copy, dir->d_name, size);  // C requares nasty things

        names[i++] = copy;
	

      }
    }

    closedir(d);

    filenames ret;
    ret.names = names;
    ret.count = file_count;

    return ret;
  }

  filenames ret;
  ret.names = NULL;
  ret.count = 0;

  return ret;
}


char* readFile(char* filename){
  FILE *f = fopen(filename, "rb");
  fseek(f, 0, SEEK_END);
  long fsize = ftell(f);
  fseek(f, 0, SEEK_SET);  //same as rewind(f);

  char *string = malloc(fsize + 1);
  fread(string, fsize, 1, f);
  fclose(f);

  string[fsize] = 0;

  return string;
  
} 


 void _gladLoadGLLoader(){
   gladLoadGLLoader((GLADloadproc) glfwGetProcAddress);
 }



char* c_double_to_string(double num, int buf_size){ 
  char *output = malloc(buf_size);

  snprintf(output, buf_size, "%f", num);

  return output;
}

char* c_int_to_string(int num, int buf_size){
  char *output = malloc(buf_size);

  snprintf(output, buf_size, "%i", num);

  return output;
}
