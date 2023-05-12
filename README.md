ode-rs-0000
===========

ODE (Open Dynamics Engine) bindings for Rust yaw kinetics samples

[oyk_512x100]: https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true
![ODE][oyk_512x100]
 * https://github.com/nomissbowling/oyk/blob/master/img/oyk_512x100.png?raw=true

Now this crate is tested on ode-0.16.2 dll version.

ode.dll drawstuff.dll for x64 Windows binary compiled with -DdDOUBLE by mingw

(It may work with VC, or other platforms.)


Requirements
------------

- [ https://github.com/nomissbowling/oyk ]( https://github.com/nomissbowling/oyk )
- [ https://github.com/nomissbowling/asciiz ]( https://github.com/nomissbowling/asciiz )
- [ ode and drawstuff ]( https://ode.org/ )

to build dll

- premake4 --with-demos --only-double --with-libccd --cc=gcc --platform--x64 --os=windows codeblocks
- premake4 --with-demos --only-double --with-libccd --platform--x64 --os=windows vs2010

in the running directory

- drawstuff.dll
- ode.dll
- libstdc++-6.dll
- libgcc_s_seh-1.dll
- libwinpthread-1.dll


edit build.rs (when need)

```rust
println!("cargo:rustc-link-search=<YOUR_LINK_LIB_PATH>/oyk/ode/lib");
```


Samples
-------

```rust
use oyk::ode::*;

pub struct SimApp {
}

impl Sim for SimApp {

fn draw_objects(&mut self) {
  self.super_mut().draw_objects();
}

fn start_callback(&mut self) {
  let t_delta = &mut self.super_mut().t_delta;
  *t_delta = 0.002;
  let obgs = &mut self.super_mut().obgs;
  let m: dReal = 1.0;
  let r: dReal = 0.2;
  for i in 0..16 {
    let c: dVector4 = vec4_from_u32(COLORS[i]);
    let p: dVector3 = [(i%4) as dReal - 1.5, (i/4) as dReal - 1.5, 2.0, 1.0];
    obgs.push(ODE::mk_sphere(m, r, &c, &p));
  }
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  obgs.push(ODE::mk_sphere(0.1, 1.0, &c, &p));
  self.super_mut().start_callback();
}

fn near_callback(&mut self, o1: dGeomID, o2: dGeomID) {
  self.super_mut().near_callback(o1, o2);
}

fn step_callback(&mut self, pause: i32) {
  self.super_mut().step_callback(pause);
}

fn command_callback(&mut self, cmd: i32) {
  match cmd as u8 as char {
    'a' => {
      println!("anything to do");
    },
    _ => {}
  }
  self.super_mut().command_callback(cmd);
}

fn stop_callback(&mut self) {
  self.super_mut().stop_callback();
}

} // impl Sim for SimApp

fn main() {
  ODE::open();
  ODE::sim_loop(
    640, 480, // 800, 600,
    Some(Box::new(SimApp{})),
    b"./resources");
  ODE::close();
}
```


License
-------

MIT License

