#include "bob.h"
#include <boost/algorithm/string.hpp>
#include <algorithm>

namespace bob {
    std::string hey(std::string input) {
        boost::algorithm::trim(input);

        bool letters = std::any_of(input.begin(), input.end(),isalpha);
        bool the_same = std::all_of(input.begin(), input.end(),[](char c) {return c == toupper(c);});//up.compare(input) == 0;
        bool questionmark = input[input.size() -1] == '?';
        
        if (the_same && letters && questionmark) {
            return "Calm down, I know what I'm doing!";
        }
        else if (input.empty()){
            return "Fine. Be that way!";
        }
        else if (questionmark) {
            return "Sure.";
        }
        else if (the_same && letters) {
            return "Whoa, chill out!";
        } else {
            return "Whatever.";
        }
    }
}  // namespace bob
