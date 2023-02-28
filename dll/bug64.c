#include <stdbool.h>
#include <stdio.h>

bool lib_dll_function (unsigned int arg1, unsigned int arg2) {
  printf("----- inside lib_dll_function(%u, %u)\n", arg1, arg2);
  return true;
}