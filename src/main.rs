#![doc(html_root_url = "https://docs.rs/ode-rs-0000/0.5.5")]
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

use impl_sim::{impl_sim_fn, impl_sim_derive};

pub struct SimApp {
  cnt: usize
}

impl SimApp {

pub fn objs_info(&mut self, f: bool, s: &str) {
  let rode = self.super_get();
  let obgs = &rode.obgs;
  let l = obgs.len();
  if f || (l != self.cnt) {
    self.cnt = l;
    println!("obgs: {} in {}", self.cnt, s);
    let rode = self.super_get(); // must re get because borrow later self.cnt
    for (k, v) in &rode.mbgs {
      println!("{}: {:018p} {:?}", k, *v, rode.obgs[v].col); // same as below
/*
      match rode.find(k.to_string()) {
        Err(e) => { println!("{}", e); },
        Ok(obg) => { println!("{}: {:018p} {:?}", k, obg.body(), obg.col); }
      }
*/
    }
  }
}

}

#[impl_sim_derive(near_callback, stop_callback)]
impl Sim for SimApp {

fn draw_objects(&mut self) {
  self.objs_info(false, "draw"); // twice (after step)
  self.super_mut().draw_objects();
}

fn start_callback(&mut self) {
  let rode = self.super_mut();
  let t_delta = &mut rode.t_delta;
  *t_delta = 0.002;
  let m: dReal = 1.0;
  let r: dReal = 0.2;
  for i in 0..16 {
    let c: dVector4 = vec4_from_u32(COLORS[i]);
    let p: dVector3 = [(i%4) as dReal - 1.5, (i/4) as dReal - 1.5, 2.0, 1.0];
    rode.mk_sphere(format!("ball_{:08X}", i), m, r, &c, &p);
  }
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  rode.mk_sphere("ball_big".to_string(), 0.1, 1.0, &c, &p);
  rode.start_callback();
}

fn step_callback(&mut self, pause: i32) {
  self.objs_info(false, "step"); // twice (before draw)
  self.super_mut().step_callback(pause);
}

fn command_callback(&mut self, cmd: i32) {
  match cmd as u8 as char {
    'o' => {
      let k = "ball_big";
      match self.super_mut().find_mut(k.to_string()) {
        Err(e) => { println!("{}", e); },
        Ok(obg) => {
          println!("{}: {:018p} {:?}", k, obg.body(), obg.col);
          println!(" pos: {}", obg.pos_vec());
          println!(" rot: {}", obg.rot_mat3());
          let pos: &mut [dReal] = obg.pos_(); // re get mut
          pos[0] += 0.2;
          pos[1] += 0.2;
          pos[2] = 5.0;
        }
      }
    },
    'a' => {
      self.objs_info(true, "cmd");
    },
    _ => {}
  }
  self.super_mut().command_callback(cmd);
}

} // impl Sim for SimApp

fn main() {
  ODE::open();
  ODE::sim_loop(
    640, 480, // 800, 600,
    Some(Box::new(SimApp{cnt: 0})),
    b"./resources");
  ODE::close();
}
