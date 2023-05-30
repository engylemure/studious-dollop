#include <stdio.h>
#include <stdlib.h>
#include <assert.h>


int cmpfunc (const void * a, const void * b) {
   return ( *(long long int*)a - *(long long int*)b);
}

int main() {
  const long long int SIZE = 100000000;
  long long int* values = malloc(sizeof(long long int)*SIZE);
  long long int* expected = malloc(sizeof(long long int)*SIZE);
  int i;
  for(i = 0; i < SIZE; i++) {
    values[i] = SIZE - (i + 1);
    expected[i] = i;
  }
  qsort(values, SIZE, sizeof(long long int), cmpfunc);
  for(i = 0; i < SIZE; i++) {
    assert(values[i] == expected[i]);
  }
  free(values);
  free(expected);
}