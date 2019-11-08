#include "gigasecond.h"

namespace gigasecond {
    ptime advance(ptime input) {
        return input + seconds(1000000000);
    }
}  // namespace gigasecond
