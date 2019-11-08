#include "isogram.h"
#include "iostream"
#include "map"
#include "cstring"

namespace isogram {
    bool is_isogram(std::string word) {
        std::map<char,int> letters;
        for(char& c: word) {
            c = tolower(c);
            if( letters[c] > 0 && 
                c != ' ' && c != '-') {
                return false;
            }

            letters[c] = 1;
        }
        return true;
    }
}  // namespace isogram
