use crate::ui::*;

mod ioi;
mod ioi_finish;

/// A UI that, using curses, prints to screen a cool animation with the status
/// of the evaluation.
///
/// The first message of the UI will choose the UI to use.
pub struct CursesUI {
    /// The underlying UI, different for each task format.
    ui: Option<Box<UI>>,
}

impl CursesUI {
    /// Make a new CursesUI.
    pub fn new() -> CursesUI {
        CursesUI { ui: None }
    }
}

impl UI for CursesUI {
    fn on_message(&mut self, message: UIMessage) {
        if let UIMessage::IOITask { .. } = message {
            self.ui = Some(Box::new(ioi::IOICursesUI::new(message).unwrap()));
        } else {
            self.ui
                .as_mut()
                .expect("Received message before the task")
                .on_message(message);
        }
    }

    fn finish(&mut self) {
        self.ui
            .as_mut()
            .expect("Received finish before the task")
            .finish();
    }
}
