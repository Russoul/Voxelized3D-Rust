#ifndef UTIL_H
#define UTIL_H

struct filenames_struct{
  char **names;
  int count;
};

typedef struct filenames_struct filenames;

filenames find_all_files_in_dir(char *path);
char* readFile(char* filename);

#endif
