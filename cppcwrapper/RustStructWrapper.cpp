#include "RustStructWrapper.h"
#include <iostream>
#include "../rustcwrapper/include/rust_c_wrapper.h"


RustStructWrapper::RustStructWrapper() {
    std::cout << "RustStructWrapper::RustStructWrapper: called" << std::endl;
    plugin = create_plugin();
}
RustStructWrapper::~RustStructWrapper() {
    std::cout << "RustStructWrapper::~RustStructWrapper: called" << std::endl;
    // TODO: clean up plugin
    // plugin;
}

void RustStructWrapper::emptyCall() {
    std::cout << "RustStructWrapper::emptyCall: called" << std::endl;
    myruststruct_empty_call(plugin);
}

void RustStructWrapper::setInt(int intToSet) {
    std::cout << "RustStructWrapper::setInt: called with: " << intToSet << std::endl;
    myruststruct_set_int(plugin, intToSet);
}

int RustStructWrapper::getInt() {
    int result = myruststruct_get_int(plugin);
    std::cout << "RustStructWrapper::getInt: called. Return value: " << result << std::endl;
    return result;
}

void RustStructWrapper::setString(const std::string stringToSet) {
    std::cout << "RustStructWrapper::setString: called" << std::endl;
    myruststruct_set_string(plugin, stringToSet.c_str());
}

std::string RustStructWrapper::getString() {
    std::cout << "RustStructWrapper::getString: called" << std::endl;
    const char* result = myruststruct_get_string(plugin);
    std::string resultString(result);
    deallocate_rust_string(result);
    return resultString;
}

void RustStructWrapper::setVector(const std::vector<std::string> &vectorToSet) {
    std::cout << "RustStructWrapper::setVector: called" << std::endl;;
    std::vector<const char*> vectorOfPointers;
    for (const auto &value : vectorToSet) {
        std::cout << value << ", ";
        vectorOfPointers.push_back(value.c_str());
    }
    std::cout << std::endl;

    myruststruct_set_vector(plugin, vectorOfPointers.data(), vectorOfPointers.size());
}

std::vector<std::string> RustStructWrapper::getVector() {
    std::cout << "RustStructWrapper::getVector: called" << std::endl;
    const char **result = myruststruct_get_vector(plugin);
    if (result == nullptr) {
        return {};
    }

    std::vector<std::string> resultVector;
    int index = 0;
    auto entry = result[index];
    while (entry != nullptr) {
        resultVector.emplace_back(entry);
        entry = result[++index];
    }

    return resultVector;
}
