#![no_std]
#![feature(codeview_annotation)]

use wdf::{
    driver_entry, wpp_control_guids, DriverObject, IoQueue, NtResult, Request, UnicodeString,
};

// Boilerplate declaration required by the tracing framework.
// SampleDriver: Name of the driver used when registering
//               with the tracing framework
// GUID:        Identity of the driver in the tracing framework
// IO and INIT: "keywords" used to categorize trace messages
wpp_control_guids!(
    SampleDriver 91e9d7e8-fc4b-4bb5-bfc7-b588a9dc0ca5 {
        IO,
        INIT,
    }
);

// A driver callback that will be called by the OS when there's
// an I/O request
#[allow(dead_code)]
fn evt_io_default(_queue: &IoQueue, _request: Request) {
    // Handle IO here...

    // Do some logging (using dummy values)
    let byte_count: usize = 1024;
    let elapsed_ms: f64 = 12.34;

    // `trace!()` will expand to a call to `codeview_annotation` which
    // embeds decoding metadata into the PDB plus a call to 
    // another function `etw::write` which will emit the values of
    // `byte_count` and `elapsed_ms` into the tracing infra at runtime
    trace!(INFO, IO, "Bytes {}, duration {} ms", byte_count, elapsed_ms);
    //     ^^^^  ^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^
    //      |    |             |                     |           |
    //  Trace  Keyword      Format string           Arg1        Arg2
    //  Level  (category)
}

// The entry-point of the driver
#[driver_entry(trace_providers(SampleDriver))]
fn driver_entry(_driver_object: &mut DriverObject, _registry_path: &UnicodeString) -> NtResult<()> {
    // Set up device here...
    // Set up I/O queues here...

    Ok(())
}
