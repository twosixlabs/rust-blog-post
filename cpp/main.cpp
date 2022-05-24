#include <iostream>
#include <vector>
#include <string>

#include "../cppcwrapper/RustStructWrapper.h"

int main(int argc, char **argv) {
    std::cout << "main: Starting " << argv[0] << std::endl;

    RustStructWrapper rustWrapper;

    rustWrapper.emptyCall();

    rustWrapper.setInt(27);
    int myIntFromRust = rustWrapper.getInt();
    std::cout << "main: got this int from rust: " << myIntFromRust << std::endl;

    rustWrapper.setString("some string from C++");
    std::string myStringFromRust = rustWrapper.getString();
    std::cout << "main: got this string from rust: " << myStringFromRust << std::endl;

    std::vector<std::string> myVectorFromCpp = {"some value", "a second value",
                                                "value number three", "fourth and final value"};
    rustWrapper.setVector(myVectorFromCpp);

    std::vector<std::string> myVectorFromRust = rustWrapper.getVector();
    if (myVectorFromRust.size() > 0) {
        std::cout << "main: got this vector from rust with these entries:"<< std::endl;
        for (const std::string &stringInVector : myVectorFromRust) {
            std::cout << "main:    " << stringInVector << std::endl;
        }
    } else {
        std::cout << "main: got an empty vector from rust."<< std::endl;
    }

    return 0;
}
