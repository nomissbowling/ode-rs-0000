#![doc(html_root_url = "https://docs.rs/ode-rs-0000/0.2.4")]
/*
  cc-rs https://crates.io/crates/cc
  bindgen https://crates.io/crates/bindgen

  dependencies asciiz oyk

  in the current directory
    drawstuff.dll
    ode.dll
    libstdc++-6.dll
    libgcc_s_seh-1.dll
    libwinpthread-1.dll
*/

use oyk::ode::*;

fn start_callback(rode: &mut ODE) {
  let t_delta = &mut rode.t_delta;
  *t_delta = 0.002;
  let obgs = &mut rode.obgs;
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
  default_start_callback(rode);
}

fn step_callback(rode: &mut ODE, pause: i32) {
  default_step_callback(rode, pause);
}

fn command_callback(rode: &mut ODE, cmd: i32) {
  match cmd as u8 as char {
    'a' => {
      println!("anything to do");
    },
    _ => {}
  }
  default_command_callback(rode, cmd);
}

fn main() {
  ODE::open();
  ODE::sim_loop(
    800, 600,
    Some(start_callback),
    None, // near_callback
    Some(step_callback),
    Some(command_callback),
    None, // stop_callback
    b"./resources");
  ODE::close();
}
