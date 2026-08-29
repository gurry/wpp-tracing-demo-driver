# wpp-tracing-demo-driver

A minimal Windows driver demonstrating [WPP software tracing](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/wpp-software-tracing) support in Rust.

## Building

### Prerequisites

| Component | Version | How to obtain |
|-----------|---------|---------------|
| Windows SDK | 10.0.26100.0 | https://learn.microsoft.com/en-us/windows-hardware/drivers/other-wdk-downloads |
| WDK | 10.0.26100.0 | https://learn.microsoft.com/en-us/windows-hardware/drivers/other-wdk-downloads |
| clang | 17 | `winget install -i LLVM.LLVM --version 17.0.6 --force` |
| Rust toolchain | N/A | Custom build of rustc containing the `codeview_annotation` intrinsic. Contact the repo author to obtain a build or build from [this PR](https://github.com/rust-lang/rust/pull/160285) yourself |
| `cargo-expand` | `cargo install cargo-expand` | `cargo install cargo-expand` |

### Build

Assuming the custom toolchain has been added with the name `stage1` just run:
```sh
cargo +stage1 build
```


### Macro expansion

```sh
cargo +stage1 expand
```

---

## `trace!()` expansion

This section shows what the expansion of the `trace!()` macro call in the function `evt_io_default()` looks like.

Here is the source code of `evt_io_default()`:

```rust
fn evt_io_default(_queue: &IoQueue, _request: Request) {
    // Handle IO here...

    // Do some logging (using dummy values)
    let byte_count: usize = 1024;
    let elapsed_ms: f64 = 12.34;

    trace!(INFO, IO, "Bytes {}, duration {} ms", byte_count, elapsed_ms);
    //     ^^^^  ^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^
    //      |    |             |                     |           |
    //  Trace  Keyword      Format string           Arg1        Arg2
    //  Level  (category)
}
```

And below is the code that it gets expanded to. The actual expansion is much bigger; we show here only the relevant parts. Inline comments explain what each part is about.

```rust
    let byte_count: usize = 1024;
    let elapsed_ms: f64 = 12.34;
    {
        {
            // Convert args that come after fmt string to
            // their byte representations
            let __f0 = byte_count;
            let __f1 = elapsed_ms;
            let __b0 = ::wpp::WppField::as_bytes(&__f0);
            let __b1 = ::wpp::WppField::as_bytes(&__f1);

            // `__wpp_schema`  is just a wrapper over the call
            // to `codeview_annotation`. Its only purpose is to 
            // help deduce the type of args `byte_count` and 
            // `elapsed_ms` in the source code above via
            // monomorphization.  The `wpp::WppField` trait is
            // defined in our library over here: https://github.com/krishnakumar4a4/windows-drivers-rs/blob/16288b683e5f48a60950c6f5c8c0e83bbcabb2f9/crates/wpp/src/field.rs#L11
            // The `codeview_annotation` call embeds decoding
            // metadata into the PDB.
            // The event ID argument ("39561") is used by decoding tools
            // to match the metadata in the PDB with traces emitted at runtime.
            fn __wpp_schema<T0: ::wpp::WppField, T1: ::wpp::WppField>(
                __f0: &T0,
                __f1: &T1,
            ) {
                struct Args<T0: ::wpp::WppField, T1: ::wpp::WppField>;

                impl<T0: ::wpp::WppField, T1: ::wpp::WppField> CodeViewAnnotationArgs for Args<T0, T1> {
                    const Args: &[&str]: &[
                            // First three args are boilerplate
                            "WPP_EVENT",
                            "SampleDriver",
                            "91e9d7e8-fc4b-4bb5-bfc7-b588a9dc0ca5",

                            "39561",                    // event ID
                            "4",                        // Tracelevel = Information
                            "IO",                       // Keyword (used to categorize the trace message)
                            "Bytes {}, duration {} ms", // Format string
                            T0::TYPE_NAME,              // Type of arg0 (e.g. "u64")
                            T1::TYPE_NAME,              // Type of arg1 (e.g. "f64")
                        ],
                }
                
                // The call to the rustc intrinsic we are proposing.
                // It gets lowered to the `llvm.codeview.annotation`
                // already present in LLVM and writes all its args
                // to the PDB as debug record of type `S_ANNOTATION`
                core::hint::codeview_annotation::<Strings::<T0, T1>>();                
            }


            // The call to `__wpp_schema` that in turn calls `codeview_annotation`
            // It has no existence at runtime
            __wpp_schema(&__f0, &__f1);

            // Call to `wpp::etw::write`. This function executes
            // at runtime and actually emits traces into the OS's trace
            // infrastructure.
            // If someone wants to view these traces they use decoding
            // tools. These tools combine the runtime traces emitted by
            // `wpp::etw::write` with the metadata embedded in the PDB
            // by `__wpp_schema` above.
            // Note the `EVENT_DESCRIPTER::Id` field with value 39561.
            // This is passed to `wpp::etw::write`. It is the same ID
            // that was passed to `codeview_annotation` call above. It
            // helps the decoding tool match the runtime traces with
            // the metadata in the PDB and construct a human readable log
            {
                let __wpp_kw_val: u64 = crate::SampleDriver::IO; // = 1u64
                if crate::SampleDriver::STATE.is_enabled(__WPP_LEVEL, __wpp_kw_val) {
                    const __WPP_EVT_DESC: ::wpp::etw::EVENT_DESCRIPTOR = ::wpp::etw::EVENT_DESCRIPTOR {
                        Id: 39561u16,
                        Version: 0,
                        Channel: 0,
                        Level: __WPP_LEVEL,
                        Opcode: 0,
                        Task: 0,
                        Keyword: crate::SampleDriver::IO,
                    };
                    // Scatter-gather list: one EVENT_DATA_DESCRIPTOR per argument.
                    let __wpp_data: [::wpp::etw::EVENT_DATA_DESCRIPTOR; 2u32 as usize] = [
                        ::wpp::etw::EVENT_DATA_DESCRIPTOR {
                            Ptr: __b0.as_ptr() as u64,
                            Size: __b0.len() as u32,
                            Reserved: 0,
                        },
                        ::wpp::etw::EVENT_DATA_DESCRIPTOR {
                            Ptr: __b1.as_ptr() as u64,
                            Size: __b1.len() as u32,
                            Reserved: 0,
                        },
                    ];

                    // `wpp::etw::write`, the function that actually
                    // emits the traces
                    unsafe {
                        ::wpp::etw::write(
                            crate::SampleDriver::STATE.reg_handle(), // Defined in the expansion that's been omitted
                            &__WPP_EVT_DESC,
                            2u32,
                            __wpp_data.as_ptr(),
                        );
                    }
                }
            }

            // Some other code here that's omitted..

        };
    };
}
