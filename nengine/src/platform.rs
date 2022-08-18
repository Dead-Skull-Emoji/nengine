// The platform module contains all the code that are only specific to a cert-
// ain operating system, for example, the window stuff. It has multiple submo-
// dules, each corresponding to each operating system we wanted to support.

// Linux stuff
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

// Windows stuff
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;