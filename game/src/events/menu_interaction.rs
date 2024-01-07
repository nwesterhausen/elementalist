use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Actions that can be taken in the menu.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    /// Move up a list. This should select the previous item in a vertical list.
    ///
    /// When at the top of a list, this should wrap to the bottom.
    Up,
    /// Move down a list. This should select the next item in a vertical list.
    ///
    /// When at the bottom of a list, this should wrap to the top.
    Down,
    /// Select the currently highlighted item.
    ///
    /// This should be used to select a menu item, or to confirm a selection.
    ///
    /// - On "checkbox" style menu items, this should toggle the checkbox.
    /// - On other menu items, this should "go into" that item so that Left/Right
    /// can be used to change the value of the item.
    /// - On "back" menu items, this should go back to the previous menu.
    Select,
    /// Go back or cancel a selection.
    ///
    /// This should be used to go back to the previous menu, or to cancel a selection.
    ///
    /// - Pressed while nothing is selected should go back to the previous menu (or exit)
    /// - Pressed while a menu item is selected should cancel the selection.
    Back,
}
