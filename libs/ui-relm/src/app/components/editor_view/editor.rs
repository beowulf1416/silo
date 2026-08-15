use tracing::debug;

use gtk::{Widget, prelude::*};
use relm4::{FactorySender, prelude::*};

// use crate::app::components::
use crate::app::components::editor_view::TabInputMsg;

pub trait Editor: std::fmt::Debug {
    pub fn content_type(&self) -> String;
    pub fn build_widget(&self, sender: FactorySender<Tab>) -> gtk::Widget;
}

#[derive(Debug)]
pub struct TextEditor {
    buffer: gtk::TextBuffer,
}

impl TextEditor {
    fn new() -> Self {
        return Self {
            buffer: gtk::TextBuffer::new(None),
        };
    }
}

impl Editor for TextEditor {
    fn content_type(&self) -> String {
        return "text/plain".to_string();
    }

    fn build_widget(&self, sender: FactorySender<Tab>) -> gtk::Widget {
        let view = gtk::TextView::builder().buffer(&self.buffer).build();

        self.buffer.connect_changed(move |buffer| {
            let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
            sender.input(TabInputMsg::ContentChanged(text));
        });

        let sw = gtk::ScrolledWindow::builder()
            .child(&view)
            .vexpand(true)
            .hexpand(true)
            .build()
            .upcast::<Widget>();
        return sw;
    }
}
