## Sample Rust for Washington SDK crate

## Notes
* `.metadata` contains a metadata-generating MSBuild project (start poking around here!)
* `.windows/winmd` is the output target for the metadata project
* `crates/samples/impl` demonstrates consuming the library crate
* `build.rs` generates Rust bindings from metadata and outputs to `src/bindings.rs`

## Building metadata
1. Install the latest production [.NET SDK](https://dotnet.microsoft.com/en-us/download/dotnet/latest).
2. Open a Developer Command Prompt for Visual Studio
3. Navigate to `.metadata` folder
4. Issue command: `dotnet build`

## Generating Rust crate
1. Ensure metadata is available in `.windows/winmd`
2. `cargo build`

## Additional resources/examples
* Generating metadata for use with the windows crate for Rust https://withinrafael.com/2023/01/18/generating-metadata-for-the-windows-crate
* Rust for DIA SDK https://github.com/microsoft/dia-rs/
* Win32 Metadata https://github.com/microsoft/win32metadata
   * Microsoft.Windows.WinmdGenerator https://github.com/microsoft/win32metadata/tree/main/sources/GeneratorSdk
