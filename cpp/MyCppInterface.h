#ifndef __CPP_MY_CPP_INTERFACE__
#define __CPP_MY_CPP_INTERFACE__

#include <string>
#include <vector>
#include "MyStruct.h"

class MyCppInterface {
public:
    virtual void emptyCall() = 0;

    // virtual void setInt(int intToSet) = 0;
    // virtual int getInt() = 0;

    // virtual void setString(const std::string stringToSet) = 0;
    // virtual std::string getString() = 0;

    // virtual void setVector(const std::vector<std::string> vectorToSet) = 0;
    // virtual std::vector<std::string> getVector() = 0;

    // virtual void setStruct(const MyStruct &structToSet) = 0;
    // virtual MyStruct getStruct() = 0;
};

#endif