## Sample Rust for Washington SDK crate

## Notes
* `.metadata` contains a metadata-generating MSBuild project (start poking around here!)
* `.windows/winmd` is a well-known location for the `windows-metadata` crate and is the output target for the metadata project
* `crates/tools/api` uses `windows-metadata::reader::TypeReader` to generate the Rust library crate (at the root) using metadata
* `crates/samples/washington` demonstrates consuming the library crate
* `src/Microsoft/*` contains generated library crate code

## Building metadata
1. Install the latest [.NET 6.0 SDK](https://dotnet.microsoft.com/download/dotnet/6.0).
2. Open a Developer Command Prompt for Visual Studio
3. Navigate to `.metadata` folder
4. `dotnet build`

## Generating Rust crate
1. Ensure metadata is available in `.windows/winmd`
2. `cargo run -p tools_api`

## Additional resources
* Rust for DIA SDK https://github.com/microsoft/dia-rs/
* Rust for Windows App SDK https://github.com/microsoft/windows-app-rs
* Win32 Metadata https://github.com/microsoft/win32metadata
   * Microsoft.Windows.WinmdGenerator https://github.com/microsoft/win32metadata/tree/main/sources/GeneratorSdk
