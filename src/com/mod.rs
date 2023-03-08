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
    /// Messages that may be safety critical in delivery.
    Critical = 0,

    /// Messages that should have guarenteed delivery.
    Important = 1,

    /// Messages used to assert control.
    Control = 2,

    /// Messages used for notifications.
    Notification = 3,

    /// Messages used to report status.
    Status = 4,

    // Not in use
    // Reserved = 5,

    /// Default message priority.
    #[default]
    Default = 6,

    /// Unimportant messages.
    Background = 7,
}
