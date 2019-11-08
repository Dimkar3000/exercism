#include "reverse_string.h"

namespace reverse_string {
    std::string reverse_string(std::string word) {
        for (uint32_t i= 0; i< word.size() / 2; i++) {
            std::swap(word[i],word[word.size() - 1 - i]);
        }
        return word;
    }
}  // namespace reverse_string
