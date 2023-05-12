#![doc(html_root_url = "https://docs.rs/ode-rs-0000/0.4.0")]
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
