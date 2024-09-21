//! Define the `Dialog` trait which can be implemented to customize dialogs
//! and `DialogDetails` struct which can be used to show dialogs.

use egui::{Color32, Id};

use crate::*;

/// Represents a dialog.
/// Implement this trait to customize dialogs.
/// 
/// # Example
/// ```
/// use egui::{Align2, Id, Window};
/// use egui_dialogs::Dialog;
/// 
/// // custom dialog for name confirmation
/// pub struct NameConfirmDialog {
///   name: String,
/// }
///
/// impl NameConfirmDialog {
///   pub fn new(name: String) -> Self {
///     Self { name }
///   }
/// }
///
/// // implement dialog logic
/// impl Dialog<String> for NameConfirmDialog {
///   fn show(&mut self, ctx: &egui::Context, _id: Option<Id>) -> Option<String> {
///     // return None if the user hasn't replied
///     let mut res = None;
///
///     // draw the dialog
///     Window::new("Confirm name")
///       .collapsible(false)
///       .resizable(false)
///       .pivot(Align2::CENTER_CENTER)
///       .show(ctx, |ui| {
///         ui.label("Your name: ");
///         ui.text_edit_singleline(&mut self.name);
///         if ui.button("Done").clicked() {
///           // set the reply and end the dialog
///           res = Some(self.name.clone());
///         }
///       });
///
///     res
///   }
/// }
/// ```
pub trait Dialog<Reply> {
    fn show(&mut self, ctx: &egui::Context, id: Option<Id>) -> Option<Reply>;
}

/// Details of a dialog to be shown and replied.
/// Used to build and show dialogs.
/// 
/// # Example
/// ```
/// use egui_dialogs::{DialogDetails, StandardReply};
///
/// # use egui_dialogs::Dialogs;
/// #
/// # pub struct MyApp<'a> {
/// #     // ... your other app states
/// #     dialogs: Dialogs<'a>,
/// # }
/// #
/// # impl MyApp<'_> {
/// #     // ... your other app logic
/// #
/// #     pub fn update(&mut self, ctx: &egui::Context) {
/// #         self.dialogs.show(ctx);
/// #
/// #         // ... your other rendering logic
/// // show a confirm dialog
/// // in your update function
/// DialogDetails::confirm("Confirm", "Are you sure you want to do this?")
///     .on_reply(|res| {
///         if res == StandardReply::Yes {
///             println!("User confirmed!");
///         }
///     })
///    .show(&mut self.dialogs);
/// #     }
/// # }
/// ```
pub struct DialogDetails<'a, Reply>
where Reply: 'a {
    pub(crate) dialog: Box<dyn Dialog<Reply> + 'a>,
    pub(crate) handler: Option<Box<dyn FnOnce(Reply) + 'a>>,
    pub(crate) mask: Option<Color32>,
    pub(crate) id: Option<Id>,
}

impl<'a, Reply> DialogDetails<'a, Reply> {
    #[inline]
    pub fn new(dialog: impl Dialog<Reply> + 'a) -> Self
    {
        Self::new_dyn(Box::new(dialog))
    }
    
    pub fn new_dyn(dialog: Box<dyn Dialog<Reply> + 'a>) -> Self {
        Self {
            dialog,
            handler: None,
            mask: Some(Color32::from_black_alpha(0x80)),
            id: None,
        }
    }

    #[inline]
    pub fn on_reply(self, handler: impl FnOnce(Reply) + 'a) -> Self {
        self.on_reply_dyn(Box::new(handler))
    }

    #[inline]
    pub fn on_reply_dyn(mut self, handler: Box<dyn FnOnce(Reply) + 'a>) -> Self {
        self.handler = Some(handler);
        self
    }

    #[inline]
    /// Set whether to show a mask over the background.
    /// The mask will intercept all user interactions with the background.
    pub fn with_mask(mut self, mask: Option<Color32>) -> Self {
        self.mask = mask;
        self
    }

    #[inline]
    /// Check if a mask is set and return it if there is.
    pub fn mask(&self) -> Option<Color32> {
        self.mask
    }

    #[inline]
    /// Set the id of the dialog. Used for identify different dialogs
    /// with a AbstractDialog trait object.
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    #[inline]
    /// Check if an id is set and return it if there is.
    pub fn id(&self) -> Option<Id> {
        self.id
    }

    pub fn show(self, dialogs: &mut Dialogs<'a>) {
        dialogs.add(self);
    }
}

/// Alias for `DialogDetails<StandardReply>`
pub type StandardDialogDetails<'a> = DialogDetails<'a, StandardReply>;

impl StandardDialogDetails<'_> {
    #[inline]
    pub fn info(title: impl Into<String>, message: impl Into<String>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::info(title.into(), message.into())
        )
    }

    #[inline]
    pub fn success(title: impl Into<String>, message: impl Into<String>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::success(title.into(), message.into())
        )
    }

    #[inline]
    pub fn confirm(title: impl Into<String>, message: impl Into<String>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::confirm(title.into(), message.into())
        )
    }

    #[inline]
    pub fn warning(title: impl Into<String>, message: impl Into<String>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::warning(title.into(), message.into())
        )
    }

    #[inline]
    pub fn error(title: impl Into<String>, message: impl Into<String>) -> Self {
        StandardDialogDetails::new(
            StandardDialog::error(title.into(), message.into())
        )
    }
}