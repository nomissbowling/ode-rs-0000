#![doc(html_root_url = "https://docs.rs/ode-rs-0000/0.8.0")]
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
            let mgm = rode.get_mgm_mut(geom).unwrap(); // must care ok_or
            mgm.get_tcm_mut().col = [1.0, 0.0, 0.0, 0.8];
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

/// create test balls
pub fn create_test_balls(&mut self) {
  let rode = self.super_mut();
  let m: dReal = 0.8;
  let r: dReal = 0.2;
  for i in 0..16 {
    let c: dVector4 = vec4_from_u32(COLORS[i]);
    let p: dVector3 = [(i%4) as dReal - 1.5, (i/4) as dReal - 1.5, 2.0, 1.0];
    let mib = MetaSphere::new(m, r, KRP095, 0, c);
    let (body, _, _) = rode.creator_m(format!("ball_{:08X}", i).as_str(), mib);
    rode.get_mut(body).expect("fail reg").set_pos(p);
  }
}

/// create test ball big
pub fn create_test_ball_big(&mut self) {
  let rode = self.super_mut();
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  let mib = MetaSphere::new(0.08 / (125.0 * PIt4), 1.0, KRP095, 0, c);
  let (body, _, _) = rode.creator("ball_big", mib);
  rode.get_mut(body).expect("fail reg").set_pos(p);
}

/// create test box small
pub fn create_test_box_small(&mut self) {
  let rode = self.super_mut();
  let mibox_small = MetaBox::new(0.1, [1.0, 1.0, 1.0, 0.0],
    KRP095, 0, [0.0, 1.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("box_small", mibox_small);
  rode.get_mut(body).expect("fail reg").set_pos([-5.0, 5.0, 2.0, 1.0]);
}

/// create test box frames
pub fn create_test_box_frames(&mut self) {
  let rode = self.super_mut();

  let mibox_big_0 = MetaBox::new(0.1, [1.0, 5.0, 0.5, 0.0],
    KRP095, 0, [1.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("box_big_0", mibox_big_0);
  rode.get_mut(body).expect("fail reg").set_pos([-9.0, -11.0, 2.0, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], PIx));

  let mibox_big_1 = MetaBox::new(0.1, [1.0, 12.0, 0.5, 0.0],
    KRP095, 0, [0.0, 1.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator("box_big_1", mibox_big_1);
  rode.get_mut(body).expect("fail reg").set_pos([12.0, -12.0, 2.0, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], -PIq));

  let mibox_big_2 = MetaBox::new(0.1, [1.0, 12.0, 0.5, 0.0],
    KRP095, 0, [0.0, 1.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("box_big_2", mibox_big_2);
  rode.get_mut(body).expect("fail reg").set_pos([12.0, 12.0, 2.0, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], PIq));

  let mibox_big_3 = MetaBox::new(0.1, [1.0, 12.0, 0.5, 0.0],
    KRP095, 0, [0.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("box_big_3", mibox_big_3);
  rode.get_mut(body).expect("fail reg").set_pos([-12.0, 12.0, 2.0, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], -PIq));
}

/// create test capsule frames
pub fn create_test_capsule_frames(&mut self) {
  let rode = self.super_mut();

  let micap_0 = MetaCapsule::new(0.001, 0.5, 16.0,
    KRP080, 0, [0.0, 1.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator("capsule_0", micap_0);
  rode.get_mut(body).expect("fail reg").set_pos([-8.6, 0.0, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([1.0, 0.0, 0.0], PIh));

  let micap_1 = MetaCapsule::new(0.001, 0.5, 16.0,
    KRP080, 0, [0.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("capsule_1", micap_1);
  rode.get_mut(body).expect("fail reg").set_pos([8.6, 0.0, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([1.0, 0.0, 0.0], PIh));
}

/// create test cylinder frames
pub fn create_test_cylinder_frames(&mut self) {
  let rode = self.super_mut();

  let micyl_0 = MetaCylinder::new(0.001, 0.5, 16.0,
    KRP080, 0, [1.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("cylinder_0", micyl_0);
  rode.get_mut(body).expect("fail reg").set_pos([0.0, 8.6, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 1.0, 0.0], PIh));

  let micyl_1 = MetaCylinder::new(0.001, 0.5, 16.0,
    KRP080, 0, [0.0, 1.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("cylinder_1", micyl_1);
  rode.get_mut(body).expect("fail reg").set_pos([0.0, -8.6, 1.5, 1.0])
    .set_rot(dMatrix3::from_axis_and_angle([0.0, 1.0, 0.0], PIh));
}

/// create test composite
pub fn create_test_composite(&mut self) {
  let rode = self.super_mut();

  let micmp_0 = MetaComposite::new(
    vec![
      MetaBox::new(0.1, [0.5, 0.5, 0.5, 0.0], KRP095, 0, [1.0, 0.0, 0.0, 0.8]),
      MetaBox::new(0.1, [0.5, 0.5, 0.5, 0.0], KRP095, 0, [0.0, 0.0, 1.0, 0.8]),
      MetaSphere::new(0.6 / PI, 0.5, KRP095, 0, [0.0, 1.0, 0.0, 0.8]),
      MetaSphere::new(0.0001, 0.1, KRPnk, 0, [1.0, 0.0, 1.0, 0.8])],
    vec![QI, QI, QI, QI],
    vec![
      [-0.4, -0.4, -0.4, 1.0],
      [0.4, 0.4, 0.4, 1.0],
      [0.0, 0.0, 0.0, 1.0],
      [0.0, 0.0, 0.0, 1.0]],
    KRPnk, 0, [1.0, 0.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator_composite("composite_0", micmp_0);
  rode.get_mut(body).expect("fail reg").set_pos([-12.0, -2.0, 2.0, 1.0])
    .set_quaternion(dQuaternion::from_axis_and_angle([0.0, 0.0, 1.0], -PIq3));

  let micmp_1 = MetaComposite::new(
    vec![
      MetaBox::new(0.1, [0.5, 0.5, 0.5, 0.0], KRP095, 0, [1.0, 0.0, 0.0, 0.8]),
      MetaBox::new(0.1, [0.5, 0.5, 0.5, 0.0], KRP095, 0, [0.0, 0.0, 1.0, 0.8]),
      MetaSphere::new(0.6 / PI, 0.5, KRP095, 0, [0.0, 1.0, 0.0, 0.8])],
    vec![
      dQuaternion::from_axis_and_angle([-0.707, 0.707, 0.0], PIq),
      dQuaternion::from_axis_and_angle([0.707, -0.707, 0.0], -PIq),
      dQuaternion::new()],
    vec![
      [-0.4, -0.4, -0.4, 1.0],
      [0.4, 0.4, 0.4, 1.0],
      [0.0, 0.0, 0.0, 1.0]],
    KRP100, 0, [1.0, 0.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator_composite("composite_1", micmp_1);
  rode.get_mut(body).expect("fail reg").set_pos([-12.0, 0.0, 2.0, 1.0])
    .set_quaternion(dQuaternion::from_axis_and_angle([0.0, 0.0, 1.0], -PIq3));
}

/// create test custom
pub fn create_test_custom(&mut self) {
  let rode = self.super_mut();

  let mitmv_cus_0 = MetaTriMesh::new(false, 0.1, unsafe { &mut *custom::tmv },
    KRP095, 0, [1.0, 0.5, 0.5, 0.8]);
  let (body, _, _) = rode.creator("tmv_cus_0", mitmv_cus_0);
  rode.get_mut(body).expect("fail reg").set_pos([-13.0, 6.0, 2.0, 1.0]);

  let mifvp_cus_0 = MetaConvex::new(false, 0.1, unsafe { &mut *custom::fvp },
    KRP095, 0, [0.5, 0.5, 1.0, 0.8]);
  let (body, _, _) = rode.creator("fvp_cus_0", mifvp_cus_0);
  rode.get_mut(body).expect("fail reg").set_pos([-13.0, 8.0, 2.0, 1.0]);
}

/// create test tetra
pub fn create_test_tetra(&mut self) {
  let rode = self.super_mut();

  let mitmv_tetra_0 = MetaTriMesh::new(false, 0.1, unsafe { &mut *tetra::tmv },
    KRP095, 0, [1.0, 0.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator("tmv_tetra_0", mitmv_tetra_0);
  rode.get_mut(body).expect("fail reg").set_pos([-13.0, -6.0, 2.0, 1.0]);

  let mifvp_tetra_0 = MetaConvex::new(false, 0.1, unsafe { &mut *tetra::fvp },
    KRP095, 0, [0.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("fvp_tetra_0", mifvp_tetra_0);
  rode.get_mut(body).expect("fail reg").set_pos([-13.0, -8.0, 2.0, 1.0]);
}

/// create test cube
pub fn create_test_cube(&mut self) {
  let rode = self.super_mut();

  let mitmv_cube_0 = MetaTriMesh::new(false, 0.1, unsafe { &mut *cube::tmv },
    KRP095, 0, [1.0, 1.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator("tmv_cube_0", mitmv_cube_0);
  rode.get_mut(body).expect("fail reg").set_pos([-7.0, 1.0, 2.0, 1.0]);

  let mifvp_cube_0 = MetaConvex::new(false, 0.1, unsafe { &mut *cube::fvp },
    KRP095, 0, [1.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("fvp_cube_0", mifvp_cube_0);
  rode.get_mut(body).expect("fail reg").set_pos([-7.0, -1.0, 2.0, 1.0]);
}

/// create test icosahedron
pub fn create_test_icosahedron(&mut self) {
  let rode = self.super_mut();

  let mitmv_icosahedron_0 = MetaTriMesh::new(false, 0.1,
    unsafe { &mut *icosahedron::tmv },
    KRP095, 0, [0.0, 1.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("tmv_icosahedron_0", mitmv_icosahedron_0);
  rode.get_mut(body).expect("fail reg").set_pos([-7.0, 3.0, 2.0, 1.0]);

  let mifvp_icosahedron_0 = MetaConvex::new(false, 0.1,
    unsafe { &mut *icosahedron::fvp },
    KRP095, 0, [1.0, 1.0, 0.0, 0.8]);
  let (body, _, _) = rode.creator("fvp_icosahedron_0", mifvp_icosahedron_0);
  rode.get_mut(body).expect("fail reg").set_pos([-7.0, -3.0, 2.0, 1.0]);
}

/// create test bunny
pub fn create_test_bunny(&mut self) {
  let rode = self.super_mut();
  let q = dQuaternion::from_axis_and_angle([1.0, 0.0, 0.0], PIh);

  let mitmv_bunny_0 = MetaTriMesh::new(false, 0.1,
    unsafe { &mut *bunny::tmv },
    KRP095, 0, [1.0, 0.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("tmv_bunny_0", mitmv_bunny_0);
  rode.get_mut(body).expect("fail reg").set_pos([-4.0, 2.0, 2.0, 1.0]).
    set_quaternion(q);

  unsafe { RecalcFaces(&mut *bunny::fvp as *mut convexfvp); }
  let mifvp_bunny_0 = MetaConvex::new(false, 0.1,
    unsafe { &mut *bunny::fvp },
    KRP095, 0, [0.0, 1.0, 1.0, 0.8]);
  let (body, _, _) = rode.creator("fvp_bunny_0", mifvp_bunny_0);
  rode.get_mut(body).expect("fail reg").set_pos([-4.0, -2.0, 2.0, 1.0]).
    set_quaternion(q);
}

/// create test plane
pub fn create_test_plane(&mut self) {
  let rode = self.super_mut();
  let dm: dReal = 0.1;
  let lxyz: dVector3 = [10.0, 10.0, 0.05, 0.0];
  let norm: dVector4 = [0.0, 0.0, 1.0, 0.0];
  let col: dVector4 = vec4_from_u32(COLORS[0]);
  let pos: dVector3 = [-5.0, -5.0, 5.0, 0.0];
  let mip = MetaPlane::new(dm, lxyz, norm, KRPnk, 0, col);
  let (body, _, _) = rode.creator("plane", mip);
  let q = dQuaternion::from_axis_and_angle([1.0, 1.0, 0.0], PIq);
  rode.get_mut(body).expect("fail reg").set_pos(pos)
    // .set_rot(dMatrix3::from_z_axis([0.7, 0.7, 0.0]));
    // .set_rot(dMatrix3::from_2_axes([-0.7, 0.7, 0.0], [0.7, 0.7, 0.0]));
    // .set_rot(dMatrix3::from_euler_angles(PIq, PIq, PIq));
    // .set_rot(dMatrix3::from_axis_and_angle([0.0, 0.0, 1.0], PIq));
    // .set_rot(dMatrix3::new());
    // .set_rot(dMatrix3::from_Q(dQuaternion::new()));
    // .set_rot(dQuaternion::new().to_R());
    // .set_quaternion(dMatrix3::new().to_Q());
    // .set_quaternion(dQuaternion::from_R(dMatrix3::new()));
    // .set_quaternion(dQuaternion::new());
    // .set_quaternion(q);
    .set_rot(q.to_R());
}

}

#[impl_sim_derive(draw_geom, near_callback, stop_callback)]
impl Sim for SimApp {

fn draw_objects(&mut self) {
  self.objs_info(false, "draw"); // twice (after step)
  self.super_mut().draw_objects();
}

fn start_callback(&mut self) {
  let t_delta = &mut self.super_mut().t_delta;
  *t_delta = 0.002;
  self.create_test_balls();
  self.create_test_ball_big();
  self.create_test_box_small();
  self.create_test_box_frames();
  self.create_test_capsule_frames();
  self.create_test_cylinder_frames();
  self.create_test_composite();
  self.create_test_custom();
  self.create_test_tetra();
  self.create_test_cube();
  self.create_test_icosahedron();
  self.create_test_bunny();
/*
  self.create_TmBall();
  self.create_Slope();
  self.create_SphereApple();
  self.create_SphereBall(); self.create_SphereRoll();
  self.create_UBall(); self.create_LUBall(); self.create_RUBall();
  self.create_VBall(); self.create_LVBall(); self.create_RVBall();
  self.create_IHBall(); self.create_IIBall();
  self.create_TmTetra(); self.create_Tetra();
  self.create_TmCube(); self.create_Cube();
  self.create_TmIcosahedron(); self.create_Icosahedron();
  self.create_TmBunny(); self.create_Bunny();
  self.create_TmCustom(); self.create_Custom();
  self.create_TmBunny2(); self.create_TmBunny3();
  self.create_Bunny2(); self.create_Bunny3();
*/
  self.create_test_plane();
  self.super_mut().start_callback();
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
