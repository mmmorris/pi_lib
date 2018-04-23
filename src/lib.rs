
#![crate_type = "rlib"]
#![feature(custom_derive,asm,box_syntax,box_patterns)]
#![feature(pointer_methods)]
#![feature(core_intrinsics)]
#![feature(generators, generator_trait)]
#![feature(associated_type_defaults)]
#[allow(dead_code,unused_variables,non_snake_case,unused_parens,unused_assignments,unused_unsafe,unused_imports)]

extern crate core;

pub mod slab;
pub mod rc;
pub mod ordmap;
#[macro_use]
pub mod sbtree;
pub mod asbtree;
pub mod bon;
pub mod data_view;
pub mod atom;
pub mod sinfo;
pub mod util;
