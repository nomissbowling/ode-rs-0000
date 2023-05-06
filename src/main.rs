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

use asciiz::u8z::{U8zBuf, u8zz::{CArgsBuf}};

use oyk::ode::*;

pub unsafe extern "C"
fn sim_loop(pause: i32) {
  step_callback(pause);
}

fn main() {
  ostatln!("Hello, ODE!");
  ODE::open();
unsafe {
  let t_delta: &mut dReal = &mut ode_get_mut!(t_delta);
  *t_delta = 0.002;

  let obgs: &mut Vec<Obg> = &mut ode_get_mut!(obgs);
  let gws: &mut Gws = &mut ode_get_mut!(gws);
  let m: dReal = 1.0;
  let r: dReal = 0.2;
  for i in 0..16 {
    let c: dVector4 = vec4_from_u32(COLORS[i]);
    let p: dVector3 = [(i%4) as dReal - 1.5, (i/4) as dReal - 1.5, 2.0, 1.0];
    obgs.push(Obg::mk_sphere(gws, m, r, &c, &p));
  }
  let c: dVector4 = [1.0, 1.0, 0.0, 0.8];
  let p: dVector3 = [0.0, 0.0, 10.0, 1.0];
  obgs.push(Obg::mk_sphere(gws, 0.1, 1.0, &c, &p));

  let ptt: U8zBuf = U8zBuf::from_u8array(b"./resources"); // to keep lifetime
  let mut dsfn: dsFunctions = dsFunctions{
    version: DS_VERSION,
    start: Some(start_callback), // Option<unsafe extern "C" fn()>
    step: Some(sim_loop), // Option<unsafe extern "C" fn(i32)>
    command: None, // Option<unsafe extern "C" fn(i32)>
    stop: None, // Option<unsafe extern "C" fn()>
    path_to_textures: ptt.as_i8p()
  };

  let mut cab: CArgsBuf = CArgsBuf::from(&std::env::args().collect());
  dsSimulationLoop(cab.as_argc(), cab.as_argv_ptr_mut(),
    640, 480, &mut dsfn);
}
  ODE::close();
}
