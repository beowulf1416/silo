use tracing::debug;

use gtk::prelude::{ButtonExt, *};
use relm4::factory::{DynamicIndex, FactoryComponent, FactorySender};

use crate::app::components::data_store::DataStoreMsg;

#[derive(Debug, Clone)]
struct Node {
    pub id: usize,
    pub name: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone)]
struct VisibleNode {
    pub id: usize,
    pub name: String,
    pub depth: usize,
    // pub children: Vec<VisibleNode>,
    pub has_children: bool,
    pub is_expanded: bool,
}

#[derive(Debug)]
pub enum TreeViewMsg {
    ToggleNode(usize),
}

// https://relm4.org/book/stable/efficient_ui/factory.html
#[derive(Debug, Clone)]
pub struct TreeViewNode {
    pub node: VisibleNode,
}

#[relm4::factory]
impl FactoryComponent for TreeViewNode {
    type Init = VisibleNode;
    type Input = TreeViewMsg;
    type Output = DataStoreMsg;
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 4,

            // indentation
            gtk::Box {
                set_width_request: 10,

                #[watch]
                set_margin_start: &(self.node.depth as i32) * 10,
            },

            // expand/collapse buttons
            gtk::Button {
                set_has_frame: false,
                #[watch]
                set_visible: self.node.has_children,
                #[watch]
                set_label: if self.node.is_expanded { "▼" } else { "▶" },
                connect_clicked[sender] => move |_| {
                    sender.input(TreeViewMsg::ToggleNode(self.node.id));
                },
            },
            gtk::Label {
                set_xalign: 0.0,
                #[watch]
                set_label: &self.node.name,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        return Self { node: init };
    }

    // fn init_widgets(
    //     &mut self,
    //     index: &Self::Index,
    //     root: Self::Root,
    //     returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
    //     sender: FactorySender<Self>,
    // ) -> Self::Widgets {
    //     root.set_margin_start(self.node.depth as i32 * 10);
    //     let widgets = view_output!();
    //     return widgets;
    // }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        debug!("{:?}", msg);
        // match msg {
        //     TreeViewMsg::ToggleNode(index) => {
        //         if let Some(node) = self.node.get_mut(index) {
        //             node.is_expanded = !node.is_expanded;
        //         }
        //     }
        // }
    }
}
