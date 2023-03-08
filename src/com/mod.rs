//! Communications module.

pub mod heartbeat;
pub mod startup;

/// Message priority.
/// 
/// These are mostly used to control message arbitration and not necessarily
/// ever read.
/// 
/// We will fill these in 0 to 7 as necesarry.
#[derive(Default, Copy, Clone)]
pub enum Priority {
    Critical = 0,
    Important = 1,
    Control = 2,
    #[default]
    Default = 6,
    Background = 7,
}
