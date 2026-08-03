#![no_std]
#![feature(codeview_annotation)]

use wdf::{
    driver_entry, wpp_control_guids, DriverObject, IoQueue, NtResult, Request, UnicodeString,
};

// WPP tracing requires you to declare what's called a "Control GUID".
// That is what this macro declares.
// The Control GUID is emitted in every trace call at run time (i.e. when
// the driver is running) and the same GUID is embedded in the decoding
// metadata in the PDB. The GUID is used by the decoding tools to
// match the the runtime traces with the metadata from the PDB and 
// produces human-readable trace messages.
wpp_control_guids!(
    SampleDriver 91e9d7e8-fc4b-4bb5-bfc7-b588a9dc0ca5 {
        // IO and INIT are what are called "keywords" 
        // They are used in trace statements to indicate the
        // category of the trace message.
        IO,
        INIT
    }
);

// The entry-point of the driver
#[driver_entry(trace_providers(SampleDriver))]
fn driver_entry(_driver_object: &mut DriverObject, _registry_path: &UnicodeString) -> NtResult<()> {
    // Set up I/O queues here
    Ok(())
}

// A driver callback that will be called by the OS when there's
// an I/O request
#[allow(dead_code)]
fn evt_io_default(_queue: &IoQueue, _request: Request) {
    // Handle IO here...

    // Do some logging
    let byte_count: usize = 1024;
    let elapsed_ms: f64 = 12.34;

    // `trace!()` will expand to a call to `codeview_annotation` which
    // embeds decoding metadata in the PDB plus a call to 
    // another function `WppAutolog` which will be executed at
    // runtime emit the values of `byte_count` and `elapsed_ms`
    // in the OS's tracing infrastructure
    trace!(IO, "Bytes {}, duration {} ms", byte_count, elapsed_ms);
}
