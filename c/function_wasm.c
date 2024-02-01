#ifdef __wasm32__
#include <string.h>
#include <stdlib.h>

__attribute__((import_module("fl_imps"), import_name("__console_log"))) void __console_log(char *ptr, size_t len);
__attribute__((import_module("fl_imps"), import_name("__get_input_data"))) void __get_input_data(char *ptr);
__attribute__((import_module("fl_imps"), import_name("__insert_error"))) void __insert_error(char *ptr, size_t len);
__attribute__((import_module("fl_imps"), import_name("__insert_response"))) void __insert_response(char *ptr, size_t len);

void console_log(char *s)
{
  __console_log(s, strlen(s));
}

char *get_input_data(int req_len)
{
  char *req_ptr = (char *)malloc(sizeof(char) * req_len);
  __get_input_data(req_ptr);
  return req_ptr;
}

void insert_response(char *s)
{
  __insert_response(s, strlen(s));
}

void insert_error(char *s)
{
  __insert_error(s, strlen(s));
}
#endif