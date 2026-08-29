use gtk::glib::Source;
use tracing::{debug, error};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};
use std::cell::{Ref, RefCell};
use std::sync::Arc;

use sourceview5::prelude::*;

// use crate::components::main_window::MainWindow;
use silo_plugin::ApplicationMessage;
use silo_plugin::node::{DataSourceNode, Node};

// currently executing statement
const CURRENT_STATEMENT_EXEC_TAG_NAME: &str = "current-statement-exec";
const CURRENT_STATEMENT_COLOR: &str = "#30C5FF";

#[derive(Debug, Default)]
pub struct QueryEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub(super) sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,

    // pub(super) sources: RefCell<Option<gio::ListStore>>,
    pub(super) cbo_sources: gtk::DropDown,
    pub(super) nb: gtk::Notebook,
    pub(super) view: sourceview5::View,
}

impl QueryEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }

    pub fn set_model(&self, sources: gio::ListStore) {
        self.cbo_sources.set_model(Some(&sources.clone()))
    }

    fn build_sources_drop_down(&self) {
        self.cbo_sources.set_enable_search(false);
        self.cbo_sources.set_show_arrow(true);
        self.cbo_sources.set_css_classes(&vec!["flat"]);

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, item| {
            let label = gtk::Label::builder().build();

            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("//todo gtk::ListItem");
            litem.set_child(Some(&label));
        });

        factory.connect_bind(|_, item| {
            let litem = item
                .downcast_ref::<gtk::ListItem>()
                .expect("//todo gtk::ListItem");

            let label = litem
                .child()
                .and_downcast::<gtk::Label>()
                .expect("//todo gtk::Label");

            let boxed = litem
                .item()
                .and_downcast::<glib::BoxedAnyObject>()
                .expect("//todo glib::BoxedAnyObject");

            let node_ref: Ref<Arc<dyn Node>> = boxed.borrow::<Arc<dyn Node>>();
            let node: &dyn Node = node_ref.as_ref();

            label.set_label(node.name());
        });

        self.cbo_sources.set_factory(Some(&factory));
        // }

        // return cbo;
    }

    fn build_action_bar(&self) -> gtk::ActionBar {
        let btn_save = gtk::Button::builder()
            // .label("Save")
            .icon_name("save")
            .tooltip_text("Save")
            .css_classes(vec!["btn", "flat"])
            .build();
        btn_save.connect_clicked(|_button| {
            debug!("button save clicked");
        });

        let separator = gtk::Separator::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let btn_execute = gtk::Button::builder()
            // .label("Execute")
            .icon_name("system-play-start")
            .tooltip_text("Execute")
            .css_classes(vec!["btn", "flat"])
            // .shortcut("Ctrl+Return")
            .build();
        btn_execute.connect_clicked(glib::clone!(
            #[weak(rename_to = this)]
            self,
            move |_button| {
                debug!("button execute clicked");

                if let Some(item) = this.cbo_sources.selected_item() {
                    let sql = this.get_current_statement();
                    debug!("sql: {:?}", sql);

                    if let Some(boxed) = item.downcast_ref::<glib::BoxedAnyObject>() {
                        let node_ref: Ref<Arc<dyn Node>> = boxed.borrow();
                        let node: &dyn Node = node_ref.as_ref();
                        let dsn_opt = node.into_DataSourceNode();
                        // };

                        if let Some(dsn) = dsn_opt {
                            let arc_dsn = Arc::clone(&dsn);
                            glib::MainContext::default().spawn_local(async move {
                                match arc_dsn
                                    .query("select schema_name from information_schema.schemata")
                                    .await
                                {
                                    Err(e) => {
                                        error!("query error: {}", e);
                                    }
                                    Ok(_) => {
                                        debug!("query succeeded");
                                    }
                                }
                            });
                        }
                    }
                }
            }
        ));

        self.build_sources_drop_down();

        let action_bar = gtk::ActionBar::builder()
            .hexpand(true)
            .css_classes(vec!["action-bar"])
            .build();
        action_bar.pack_start(&btn_save);
        action_bar.pack_start(&separator);
        action_bar.pack_start(&btn_execute);
        action_bar.pack_end(&self.cbo_sources);

        return action_bar;
    }

    fn build_editor(&self) -> gtk::ScrolledWindow {
        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        if let Some(ref language) = sourceview5::LanguageManager::new().language("sql") {
            buffer.set_language(Some(language));
        }

        if let Some(ref scheme) = sourceview5::StyleSchemeManager::new().scheme("solarized-light") {
            buffer.set_style_scheme(Some(scheme));
        }

        // let view = sourceview5::View::with_buffer(&buffer);
        // self.view.set_buffer(Some(&buffer));
        // self.view.set_monospace(true);
        // self.view
        //     .set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        // self.view.set_show_line_numbers(true);
        // self.view.set_highlight_current_line(true);
        // // view.set_highlight_matching_brackets(true);
        // self.view.set_tab_width(4);
        // self.view.set_hexpand(true);
        // self.view.set_vexpand(true);

        self.view.set_buffer(Some(&buffer));
        self.init_source_view();

        let sv = gtk::ScrolledWindow::builder()
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .child(&self.view)
            .build();

        return sv;
    }

    fn init_source_view(&self) {
        self.view.set_monospace(true);
        self.view
            .set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        self.view.set_show_line_numbers(true);
        self.view.set_highlight_current_line(true);
        // view.set_highlight_matching_brackets(true);
        self.view.set_tab_width(4);
        self.view.set_hexpand(true);
        self.view.set_vexpand(true);

        // set up current statement highlighting
        let tagtbl = self.view.buffer().tag_table();
        if tagtbl.lookup(CURRENT_STATEMENT_EXEC_TAG_NAME).is_none() {
            let tag = sourceview5::Tag::builder()
                .name(CURRENT_STATEMENT_EXEC_TAG_NAME)
                .background(CURRENT_STATEMENT_COLOR)
                .build();
            tagtbl.add(&tag);
        }
    }

    fn get_current_statement(&self) -> Option<String> {
        let buffer = self.view.buffer();
        let tm = buffer.get_insert();

        let cursor_iter = buffer.iter_at_mark(&tm);
        debug!("cursor_iter: {:?}", cursor_iter);

        let mut start_iter = cursor_iter;
        let mut end_iter = cursor_iter;

        // 1. Scan backward for the statement start (e.g., previous ';')
        // step backward once at the start
        start_iter.backward_char();
        while start_iter.backward_char() {
            // debug!("start_iter.char() {}", start_iter.char());
            if start_iter.char() == ';' {
                // Move past the ';' so we don't include the previous statement's trailing semicolon
                start_iter.forward_char();
                break;
            }
        }

        // 2. Scan forward for the statement end (e.g., next ';')
        while end_iter.char() != ';' {
            // debug!("end_iter.char() {}", end_iter.char());
            if !end_iter.forward_char() {
                break; // Reached end of buffer
            }
        }

        // Include the trailing semicolon if end_iter is currently pointing at one
        if end_iter.char() == ';' {
            end_iter.forward_char();
        }

        // highlight the current statement
        let (start, end) = buffer.bounds();
        buffer.remove_tag_by_name(CURRENT_STATEMENT_EXEC_TAG_NAME, &start, &end);
        buffer.apply_tag_by_name(CURRENT_STATEMENT_EXEC_TAG_NAME, &start_iter, &end_iter);

        // 3. Slice and trim text
        let text = buffer.text(&start_iter, &end_iter, false);
        let trimmed = text.trim();

        debug!("text: {} {}", text, trimmed);

        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for QueryEditor {
    const NAME: &'static str = "QueryEditor";
    type Type = super::QueryEditor;
    type ParentType = gtk::Box;
}

impl ObjectImpl for QueryEditor {
    fn constructed(&self) {
        self.parent_constructed();

        let paned = gtk::Paned::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .shrink_start_child(false)
            .resize_start_child(false)
            .build();

        let editor = self.build_editor();

        self.nb.set_hexpand(true);
        self.nb.set_vexpand(true);

        paned.set_start_child(Some(&editor));
        paned.set_end_child(Some(&self.nb));

        let action_bar = self.build_action_bar();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.append(&action_bar);
        container.append(&paned);

        let obj = self.obj();
        obj.append(&container);
    }
}

impl WidgetImpl for QueryEditor {}

impl BoxImpl for QueryEditor {}
