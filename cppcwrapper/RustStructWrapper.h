#ifndef __CPPCWRAPPER_RUSTSTRUCTWRAPPER_H__
#define __CPPCWRAPPER_RUSTSTRUCTWRAPPER_H__

#include "../cpp/MyForeignInterface.h"

class RustStructWrapper : public MyForeignInterface {
public:
    RustStructWrapper();
    virtual ~RustStructWrapper();

    void emptyCall() override;

    void setInt(int intToSet) override;
    int getInt() override;

    void setString(const std::string stringToSet) override;
    std::string getString() override;

    void setVector(const std::vector<std::string> &vectorToSet) override;
    std::vector<std::string> getVector() override;

private:
    void *plugin;
};

#endif
