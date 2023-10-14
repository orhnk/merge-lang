#include <stdio.h>

char *ccall() {

  char *message = "Hello, World!";
  printf("[C] Hello from C! 👋\n");
  printf("[C] Sending message: %s to Rust...\n", message);

  return message;
}
