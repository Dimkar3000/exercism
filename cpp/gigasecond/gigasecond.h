#if !defined(GIGASECOND_H)
#define GIGASECOND_H

#include "boost/date_time/posix_time/posix_time.hpp"
using ptime = boost::posix_time::ptime;
using seconds = boost::posix_time::seconds;   

namespace gigasecond {
    boost::posix_time::ptime advance(boost::posix_time::ptime input);
}  // namespace gigasecond

#endif // GIGASECOND_H