ode-rs-0000
===========

ODE bindings for Rust yaw kinetics samples

- edit build.rs (when need)

```rust
println!("cargo:rustc-link-search=<YOUR_LINK_LIB_PATH>/oyk/ode/lib");
```

- copy dlls in the running directory
  - drawstuff.dll
  - ode.dll
  - libstdc++-6.dll
  - libgcc_s_seh-1.dll
  - libwinpthread-1.dll


Requires
--------

- [ ode and drawstuff ]( https://ode.org/ )
- [ https://github.com/nomissbowling/oyk ]( https://github.com/nomissbowling/oyk )
- [ https://github.com/nomissbowling/asciiz ]( https://github.com/nomissbowling/asciiz )


License
-------

MIT License

