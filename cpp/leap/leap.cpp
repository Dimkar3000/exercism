#include "leap.h"

bool leap::is_leap_year(int input){
    return input % 400 == 0 || (input % 4 == 0 && input % 100 != 0);
}
