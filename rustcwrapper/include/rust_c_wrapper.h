#ifndef __RUSTCWRAPPER_INCLUDE_RUST_C_WRAPPER_H__
#define __RUSTCWRAPPER_INCLUDE_RUST_C_WRAPPER_H__


extern "C" {

void *create_plugin();

void deallocate_rust_string(const char *string_to_dealloc);

void myruststruct_empty_call(void *plugin);

void myruststruct_set_int(void *plugin, int intToSet);
int myruststruct_get_int(void *plugin);

void myruststruct_set_string(void *plugin, const char *stringToSet);
const char* myruststruct_get_string(void *plugin);

void myruststruct_set_vector(void *plugin, const char **vectorToSet, int vectorSize);
const char** myruststruct_get_vector(void *plugin);

}

#endif
