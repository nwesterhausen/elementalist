//! Enums for the progress bar.

/// The states of a progress bar.
///
/// These are used to determine the color of the bar.
pub enum BarState {
    /// The Ok state. By default this would be green.
    Ok,
    /// The Moderate state. By default this would be yellow.
    Moderate,
    /// The Critical state. By default this would be red.
    Critical,
}
