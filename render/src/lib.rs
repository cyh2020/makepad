#![allow(dead_code)]

#[macro_use]
mod cx;
#[macro_use]
mod livemacros;

#[cfg(all(not(feature="ipc"),target_os = "linux"))]
mod cx_opengl;
#[cfg(all(not(feature="ipc"),target_os = "linux"))]
mod cx_xlib;
#[cfg(all(not(feature="ipc"),any(target_os = "linux")))]
mod cx_linux;

#[cfg(all(not(feature="ipc"),target_os = "macos"))]
mod cx_metal;
#[cfg(all(not(feature="ipc"),target_os = "macos"))]
mod cx_cocoa;
#[cfg(all(not(feature="ipc"),any(target_os = "macos")))]
mod cx_macos;
#[cfg(all(not(feature="ipc"),any(target_os = "macos")))]
mod cx_apple;

#[cfg(all(not(feature="ipc"),target_os = "windows"))]
mod cx_dx11;
#[cfg(all(not(feature="ipc"),target_os = "windows"))]
mod cx_win32;
#[cfg(all(not(feature="ipc"),any(target_os = "windows")))]
mod cx_windows;

#[cfg(all(not(feature="ipc"),target_arch = "wasm32"))]
mod cx_webgl;
#[macro_use]
#[cfg(all(not(feature="ipc"),target_arch = "wasm32"))]
mod cx_wasm32;

#[macro_use]
#[cfg(all(not(feature="ipc"),any(target_os = "linux", target_os="macos", target_os="windows")))]
mod cx_desktop;

mod turtle;
mod fonts;
mod cursor;
mod window;
mod view;
mod pass;
mod texture;
mod animator;
mod elements;
mod area;
mod geometrygen;
mod quad;
mod blit;
mod text;
mod events;
mod menu; 
mod geometry;
mod shader;
mod cube;
mod shader_std;
mod gpuinfo;

pub use crate::cx::*;
pub use crate::quad::*;
pub use crate::cube::*;
pub use crate::blit::*;
pub use crate::text::*;
pub use crate::elements::*;
