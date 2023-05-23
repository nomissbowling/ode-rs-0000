#![doc(html_root_url = "https://docs.rs/ode-rs-0000/0.6.8")]
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

use oyk::colors::*;
use oyk::ode::*;

use std::ffi::{c_void}; // used by impl_sim_fn
use impl_sim::{impl_sim_fn, impl_sim_derive};

pub struct SimApp {
  cnt: usize
}

impl SimApp {

pub fn objs_mut(&mut self, f: bool, s: &str) {
  let rode = self.super_mut();
  if f || rode.is_modified(false) {
    self.cnt = rode.num();
    println!("obgs: {} in {}", self.cnt, s);
    let rode = self.super_get(); // must re get
    let ids = rode.each_id(|_key, _id| { true }); // lambda may return false
    for id in ids {
      if id == 0 as dBodyID { continue; } // skipped by result of each_id
      let rode = self.super_mut(); // must re get
      match rode.get_mut(id) {
        Err(e) => { println!("{}", e); },
        Ok(obg) => {
          // This is test code using each_id with get_mut, but high cost.
          // Better to use self.super_mut().find_mut("ball_big".to_string())
          if obg.key == "ball_big" { obg.col = [1.0, 0.0, 0.0, 0.8]; }
          println!("{}: {:018p} {:?}", obg.key, id, obg.col);
          // get_tcm_mut must be after accessing to obg members
          if obg.key == "ball_big" {
            let geom = obg.geom(); // must assign before get_tcm_mut
            let tcm = rode.get_tcm_mut(geom).unwrap(); // must care ok_or
            tcm.col = [1.0, 0.0, 0.0, 0.8];
          }
        }
      }
    }
  }
}

pub fn objs_info(&mut self, f: bool, s: &str) {
  let rode = self.super_mut();
  if f || rode.is_modified(false) {
    self.cnt = rode.num();
    println!("obgs: {} in {}", self.cnt, s);
    let rode = self.super_get(); // must re get because borrow later self.cnt
    rode.each(|key, id, obg| {
      println!("{}: {:018p} {:?}", key, id, obg.col);
      true
    });
  }
}

}

#[impl_sim_derive(draw_geom, near_callback, stop_callback)]
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
    let mib = Box::new(MetaSphere::new(m, r, 1.0, 0, c));
    let body = rode.creator(format!("ball_{:08X}", i), mib);
    rode.get_mut(body).expect("fail reg").set_pos(p);
  }
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  let mib = Box::new(MetaSphere::new(0.1, 1.0, 0.5, 0, c));
  let body = rode.creator("ball_big".to_string(), mib);
  rode.get_mut(body).expect("fail reg").set_pos(p);
  let mi = Box::new(MetaBox::new(0.1, [1.0, 1.0, 1.0, 0.0], 0.8,
    0, [0.0, 1.0, 1.0, 0.8]));
  let body = rode.creator("box_big".to_string(), mi);
  rode.get_mut(body).expect("fail reg").set_pos([-5.0, 5.0, 2.0, 1.0]);
  let micap_0 = Box::new(MetaCapsule::new(0.1, 0.5, 16.0, 0.5,
    0, [0.0, 1.0, 0.0, 0.8]));
  let body = rode.creator("capsule_big".to_string(), micap_0);
  rode.get_mut(body).expect("fail reg").set_pos([-8.6, 0.0, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([1.0, 0.0, 0.0], 1.57));
  let micyl_0 = Box::new(MetaCylinder::new(0.1, 0.5, 16.0, 0.5,
    0, [1.0, 0.0, 1.0, 0.8]));
  let body = rode.creator("cylinder_big".to_string(), micyl_0);
  rode.get_mut(body).expect("fail reg").set_pos([0.0, 8.6, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 1.0, 0.0], 1.57));
  let micap_1 = Box::new(MetaCapsule::new(0.1, 0.5, 16.0, 0.5,
    0, [0.0, 0.0, 1.0, 0.8]));
  let body = rode.creator("capsule_big".to_string(), micap_1);
  rode.get_mut(body).expect("fail reg").set_pos([8.6, 0.0, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([1.0, 0.0, 0.0], 1.57));
  let micyl_1 = Box::new(MetaCylinder::new(0.1, 0.5, 16.0, 0.5,
    0, [0.0, 1.0, 1.0, 0.8]));
  let body = rode.creator("cylinder_big".to_string(), micyl_1);
  rode.get_mut(body).expect("fail reg").set_pos([0.0, -8.6, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 1.0, 0.0], 1.57));
  let dm: dReal = 0.1;
  let lxyz: dVector3 = [10.0, 10.0, 0.05, 0.0];
  let norm: dVector4 = [0.0, 0.0, 1.0, 0.0];
  let col: dVector4 = vec4_from_u32(COLORS[0]);
  let pos: dVector3 = [-5.0, -5.0, 5.0, 0.0];
  let mip = Box::new(MetaPlane::new(dm, lxyz, norm, 1.0, 0, col));
  let body = rode.creator("plane".to_string(), mip);
  let q = dQuaternion::from_axis_and_angle([1.0, 1.0, 0.0], 0.78);
  rode.get_mut(body).expect("fail reg").set_pos(pos)
    // .set_rot(dMatrix3::from_z_axis([0.7, 0.7, 0.0]));
    // .set_rot(dMatrix3::from_2_axes([-0.7, 0.7, 0.0], [0.7, 0.7, 0.0]));
    // .set_rot(dMatrix3::from_euler_angles(0.78, 0.78, 0.78));
    // .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], 0.78));
    // .set_rot(dMatrix3::new());
    // .set_rot(dMatrix3::from_Q(dQuaternion::new()));
    // .set_rot(dQuaternion::new().to_R());
    // .set_quaternion(dMatrix3::new().to_Q());
    // .set_quaternion(dQuaternion::from_R(dMatrix3::new()));
    // .set_quaternion(dQuaternion::new());
    // .set_quaternion(q);
    .set_rot(q.to_R());
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
    'b' => {
      self.objs_mut(true, "mut");
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
