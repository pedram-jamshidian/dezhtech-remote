#include <opus/opus.h>
#include <opus/opus_defines.h>
#include <opus/opus_types.h>

// Ensure all constants are defined
#ifndef OPUS_OK
#define OPUS_OK 0
#endif

#ifndef OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST
#define OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST 4046
#endif

#ifndef OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST  
#define OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST 4047
#endif

#ifndef OPUS_GET_IN_DTX_REQUEST
#define OPUS_GET_IN_DTX_REQUEST 4049
#endif
