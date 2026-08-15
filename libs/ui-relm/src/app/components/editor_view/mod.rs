// pub mod editor;
// pub mod tab;

use tracing::debug;

use gtk::{Box, Notebook, prelude::*};

// use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent, factory::*};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender, FactoryVecDeque};
use relm4::prelude::*;

use crate::app::windows::main::MainWindowMsg;

#[derive(Debug)]
pub enum EditorViewInputMsg {
    AddEditor,
}

#[derive(Debug)]
pub struct EditorView {}

impl EditorView {
    fn build_tab_header(label: &str) -> gtk::Box {
        let th = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(4)
            // .append(&gtk::Image::builder().icon_name("folder-visiting").build())
            // .append(&gtk::Label::builder().label(label).build())
            .build();

        let close_button = gtk::Button::builder()
            .css_classes(vec!["flat", "button", "close"])
            .icon_name("window-close")
            .build();
        // close_button.add_css_class("flat");
        close_button.connect_clicked(|button| {
            debug!("close button clicked");
        });

        th.append(&gtk::Image::builder().icon_name("folder-visiting").build());
        th.append(&gtk::Label::builder().label(label).build());

        th.append(&close_button);

        return th;
    }
}

#[relm4::component(pub)]
impl SimpleComponent for EditorView {
    type Init = ();
    type Input = EditorViewInputMsg;
    type Output = MainWindowMsg;
    type Widgets = EditorViewWidgets;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_vexpand: true,
            set_hexpand: true,

            gtk::Notebook {
                set_tab_pos: gtk::PositionType::Top,
                set_hexpand: true,
                set_vexpand: true,

                append_page[Some(&Self::build_tab_header("test_1"))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 0,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 0,

                        gtk::Label {
                            set_label: "right_1",
                        }
                    },
                },

                append_page[Some(&Self::build_tab_header("test_2"))] = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 0,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 0,

                        gtk::Label {
                            set_label: "right_2",
                        }
                    },
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        return ComponentParts { model, widgets };
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            EditorViewInputMsg::AddEditor => {
                debug!("//todo EditorViewInputMsg::AddEditor");
            }
        }
    }
}
