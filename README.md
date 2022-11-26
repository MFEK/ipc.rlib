# MFEK IPC (inter-process communication) Library v0.0.4-beta1

Modular Font Editor K (MFEK) inter-process communication library. No longer alpha, now in beta ꞵ! Yet still has an unstable API, and will remain unversioned for now.

This library is responsible for:

* communication between running MFEK modules (including modules of the same type, e.g. multiple processes of MFEKglif)
* providing helper functions for running modules w/CLI APIs that often need to be called by other modules (examples: `MFEKinit`, `MFEKmetadata`) (this part might be spun off into another library)
* providing the `IPCInfo` struct which can be used by modules to tell other modules what they know about the current environment (answering questions like: is `.glif` parented? do we need to add `.exe` to module names on command line?)
* provides an API for version checking of modules and an `Available` enum, if user doesn't have the module we need (`Yes`, `No`, `Degraded` [version mismatch, might be ok, might not])
* display of ASCII art headers, such as…

              ___           ___         ___           ___
             /\  \         /\__\       /\__\         /|  |
            |::\  \       /:/ _/_     /:/ _/_       |:|  |
            |:|:\  \     /:/ /\__\   /:/ /\__\      |:|  |
          __|:|\:\  \   /:/ /:/  /  /:/ /:/ _/_   __|:|  |
         /::::|_\:\__\ /:/_/:/  /  /:/_/:/ /\__\ /\ |:|__|____           __                    __
         \:\~~\  \/__/ \:\/:/  /   \:\/:/ /:/  / \:\/:::::/__/   _____  / /_   _____  ____    / /__  ___
          \:\  \        \::/__/     \::/_/:/  /   \::/~~/~      / ___/ / __/  / ___/ / __ \  / //_/ / _ \
           \:\  \        \:\  \      \:\/:/  /     \:\~~\      (__  ) / /_   / /    / /_/ / / ,<   /  __/
            \:\__\        \:\__\      \::/  /       \:\__\    /____/  \__/  /_/     \____/ /_/|_|  \___/
             \/__/         \/__/       \/__/         \/__/
    * «unironically MFEK's killer feature, this is what will finally cause my font editor project to be taken seriously» ~ Fred 2021-12-19
    * (if you hate these, you can disable them with environment variable `MFEK_SUPPRESS_HEADER`; disable system-wide by putting in shell profile e.g. for bash `export MFEK_SUPPRESS_HEADER=1`)
