#include "hamming.h"
#include <stdexcept>
namespace hamming {
    int compute(std::string f, std::string s) {
        if(f.size() != s.size()) {
            throw std::domain_error("wrong sizes");
        }
        int result = 0;
        for(uint32_t i = 0; i < f.size(); i++) {
            if (f[i] != s[i]) {
                result++;
            }
        }
        return result;
    }
}  
