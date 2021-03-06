#![allow(dead_code)]

extern crate nalgebra as na;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use inflector::Inflector;

use rapier_testbed3d::Testbed;
use std::cmp::Ordering;

mod balls3;
mod boxes3;
mod capsules3;
mod debug_boxes3;
mod debug_triangle3;
mod domino3;
mod heightfield3;
mod joints3;
mod kinematic3;
mod pyramid3;
mod sensor3;
mod stacks3;
mod stress_joint_ball3;
mod stress_joint_fixed3;
mod stress_joint_prismatic3;
mod stress_joint_revolute3;
mod stress_keva3;
mod trimesh3;

fn demo_name_from_command_line() -> Option<String> {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        if &arg[..] == "--example" {
            return args.next();
        }
    }

    None
}

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
fn demo_name_from_url() -> Option<String> {
    None
    //    let window = stdweb::web::window();
    //    let hash = window.location()?.search().ok()?;
    //    if hash.len() > 0 {
    //        Some(hash[1..].to_string())
    //    } else {
    //        None
    //    }
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
fn demo_name_from_url() -> Option<String> {
    None
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let demo = demo_name_from_command_line()
        .or_else(|| demo_name_from_url())
        .unwrap_or(String::new())
        .to_camel_case();

    let mut builders: Vec<(_, fn(&mut Testbed))> = vec![
        ("Balls", balls3::init_world),
        ("Boxes", boxes3::init_world),
        ("Capsules", capsules3::init_world),
        ("Domino", domino3::init_world),
        ("Heightfield", heightfield3::init_world),
        ("Joints", joints3::init_world),
        ("Kinematic", kinematic3::init_world),
        ("Stacks", stacks3::init_world),
        ("Pyramid", pyramid3::init_world),
        ("Sensor", sensor3::init_world),
        ("Trimesh", trimesh3::init_world),
        ("(Debug) boxes", debug_boxes3::init_world),
        ("(Debug) triangle", debug_triangle3::init_world),
        ("(Stress test) joint ball", stress_joint_ball3::init_world),
        ("(Stress test) joint fixed", stress_joint_fixed3::init_world),
        (
            "(Stress test) joint revolute",
            stress_joint_revolute3::init_world,
        ),
        (
            "(Stress test) joint prismatic",
            stress_joint_prismatic3::init_world,
        ),
        ("(Stress test) keva tower", stress_keva3::init_world),
    ];

    // Lexicographic sort, with stress tests moved at the end of the list.
    builders.sort_by(|a, b| match (a.0.starts_with("("), b.0.starts_with("(")) {
        (true, true) | (false, false) => a.0.cmp(b.0),
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
    });

    let i = builders
        .iter()
        .position(|builder| builder.0.to_camel_case().as_str() == demo.as_str())
        .unwrap_or(0);

    let testbed = Testbed::from_builders(i, builders);

    testbed.run()
}
