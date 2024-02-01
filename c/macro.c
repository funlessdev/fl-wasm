#define VERSION 1
__attribute__((export_name("__runtime_version"))) int __runtime_version()
{
    return VERSION;
}

#ifdef __wasm32__
#include "function_wasm.c"
#include "json-c/json.h"

char *fl_main();
void console_log(char *input);
struct json_object *json_tokener_parse(const char *str);

__attribute__((export_name("__invoke"))) char *__invoke(int size)
{
    char *input = get_input_data(size);
    console_log(input);
    json_object *obj = json_tokener_parse(input);
    json_object *field = json_object_object_get(obj, "input");
    char *strField = json_object_to_json_string(field);
    console_log(strField);
    char *response = fl_main();
    insert_response(response);
    return 0;
}
#endif