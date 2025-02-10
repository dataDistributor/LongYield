#ifndef LONGYIELD_FFI_H
#define LONGYIELD_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

// Exposed function to start mining from the Rust backend.
void start_mining();

// Exposed function to get the current blockchain length.
unsigned int get_blockchain_length();

#ifdef __cplusplus
}
#endif

#endif // LONGYIELD_FFI_H
