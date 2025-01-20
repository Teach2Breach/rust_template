This is a template that I use when starting new rust projects that I want to be able to compile as an exe or a dll with sRDI compatibility for windows.

main (run if compiled to exe) and lib (run if compiled to dll) each just call the Pick function in proto.rs. So the code you want to execute should all be contained in that function (or called by it). To keep things clean, you can put additional functions in func.rs and call them from proto.rs. However, due to how rust treats functions declared in other modules, the functions in func called by proto, must be public. If you want to keep functions private, you can declare them within proto, or have a public helper function in func that calls other private functions within func. An example of each method is used to print hello world in this example.

NOTE - When testing the compiled dll with Rundll32, information printed to console will not be displayed. In order to demonstrate the dll works with a simple test with Rundll32, I also added a function to write to a file.

When compiling a project built in this manner, it is important to specify whether to compile the exe or dll, to avoid naming collisions. Example:

build exe:

```
cargo build --bin rust_template --release
```

build dll:

```
cargo build --lib --release
```

When testing the dll with Rundll32, you must specify the exported Pick function:

```
rundll32.exe .\target\release\rust_template.dll,Pick
```

Since most projects I write are used for malware dev, this project is also set up to use litcrypt. In order to compile a project using litcrypt, make sure to set an environment variable LITCRYPT_ENCRYPT_KEY. The value assigned is arbitrary, it can be anything. For more information see litcrypt documentation: https://docs.rs/litcrypt/latest/litcrypt/

This project is also purposely structured to be compatible with donut (https://github.com/TheWover/donut). To use donut to convert this project to shellcode, you can use the following syntax (change function name to match the function you want to execute):
(a2 targets 64 bit. change accordinly, but I never do anything for 32 bit these days)

```
.\donut.exe -a2 -i .\rust_template.dll --function Pick -t
```

Alternatively, you can use another of my tools dll2shell (https://github.com/Teach2Breach/dll2shell) to convert the compiled dll to shellcode.
