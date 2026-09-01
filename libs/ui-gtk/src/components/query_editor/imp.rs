use gtk::ApplicationWindow;
use tracing::{debug, error};

use gtk::{
    gio::{self, Action, prelude::*},
    glib,
    prelude::*,
    subclass::prelude::*,
};
use std::sync::Arc;
use std::{
    cell::{Ref, RefCell},
    result,
};

use sourceview5::prelude::*;

use silo_plugin::ApplicationMessage;
use silo_plugin::node::{DataSourceNode, Node, QueryResult};

use crate::get_runtime;

// currently executing statement
const CURRENT_STATEMENT_EXEC_TAG_NAME: &str = "current-statement-exec";
const CURRENT_STATEMENT_COLOR: &str = "#30C5FF";

#[derive(Debug, Default)]
pub struct QueryEditor {
    // pub window: RefCell<Option<MainWindow>>,
    pub(super) sender: RefCell<Option<async_channel::Sender<ApplicationMessage>>>,

    // pub(super) sources: RefCell<Option<gio::ListStore>>,
    pub(super) btn_execute: gtk::Button,
    pub(super) cbo_sources: gtk::DropDown,
    pub(super) nb: gtk::Notebook,
    pub(super) view: sourceview5::View,
    pub(super) data_sources: RefCell<Option<gio::ListStore>>,
}

impl QueryEditor {
    // pub fn set_main_window(&self, window: &MainWindow) {
    //     self.window.replace(Some(window.clone()));
    // }

    pub fn set_model(&self, sources: gio::ListStore) {
        self.data_sources.replace(Some(sources.clone()));

        self.cbo_sources.set_model(Some(&sources.clone()))
    }

    pub fn execute(&self) {
        debug!("executing...");
        if let Some(item) = self.cbo_sources.selected_item() {
            let sql = self.get_current_statement();

            if let Some(boxed) = item.downcast_ref::<glib::BoxedAnyObject>() {
                let node_ref: Ref<Arc<dyn Node>> = boxed.borrow();
                let node: &dyn Node = node_ref.as_ref();
                let dsn_opt = node.into_DataSourceNode();

                if let Some(dsn) = dsn_opt {
                    let arc_dsn = Arc::clone(&dsn);

                    let handle = get_runtime().spawn(async move {
                        let value = sql.unwrap_or_default();

                        let future = { arc_dsn.query(value.as_str()) };

                        match future.await {
                            Err(e) => {
                                error!("an error occured while executing query: {}", e);
                                None
                            }
                            Ok(result) => {
                                debug!("succeeded 1");
                                Some(result)
                            }
                        }
                    });

                    let obj = self.obj();

                    let action = obj
                        .root()
                        .and_then(|r| r.downcast::<ApplicationWindow>().ok())
                        .and_then(|mw| mw.lookup_action("query-execute"))
                        .and_then(|a| a.downcast::<gio::SimpleAction>().ok());

                    if let Some(action) = action {
                        action.set_enabled(false);

                        glib::MainContext::default().spawn_local(glib::clone!(
                            #[weak(rename_to = this)]
                            self,
                            async move {
                                match handle.await {
                                    Err(e) => {
                                        error!("unable to execute query :{}", e);

                                        action.set_enabled(true);
                                    }
                                    Ok(result) => {
                                        debug!("succeeded 2");
                                        this.add_result(&result);

                                        action.set_enabled(true);
                                    }
                                }
                            }
                        ));
                    } else {
                        error!("action not found");
                    }
                }
            }
        }
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

        self.btn_execute.set_action_name(Some("win.query-execute"));
        self.btn_execute.set_icon_name("system-play-start");
        self.btn_execute.set_tooltip_text(Some("Execute"));
        self.btn_execute.set_css_classes(&vec!["btn", "flat"]);

        self.build_sources_drop_down();

        let action_bar = gtk::ActionBar::builder()
            .hexpand(true)
            .css_classes(vec!["action-bar"])
            .build();
        action_bar.pack_start(&btn_save);
        action_bar.pack_start(&separator);
        action_bar.pack_start(&self.btn_execute);
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

    fn build_result_sources_dropdown(&self) -> gtk::DropDown {
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

        let sources = {
            let ref_sources = self.data_sources.borrow();
            ref_sources.as_ref().expect("gio::ListStore").clone()
        };

        let cbo = gtk::DropDown::builder()
            .enable_search(false)
            .show_arrow(true)
            .factory(&factory)
            .model(&sources)
            .build();

        return cbo;
    }

    fn build_column_view(&self, result: &Option<QueryResult>) -> gtk::ColumnView {
        let store = gio::ListStore::new::<glib::BoxedAnyObject>();
        if let Some(result) = result {
            let rows = result.rows.clone();
            for row in rows {
                store.append(&glib::BoxedAnyObject::new(row));
            }
        }

        let selection_model = gtk::SingleSelection::new(Some(store));

        let lv = gtk::ColumnView::builder()
            .model(&selection_model)
            .show_column_separators(true)
            .show_row_separators(true)
            .build();

        if let Some(result) = result {
            let columns = result.columns.clone();
            for column in columns {
                let cvc = gtk::ColumnViewColumn::builder()
                    .title(column.name.clone())
                    .build();
                lv.append_column(&cvc);
            }
        }

        return lv;
    }

    fn add_result(&self, result: &Option<QueryResult>) {
        // tab header
        let icon = gtk::Image::builder()
            .icon_name("folder-visiting-symbolic")
            .build();

        let label = gtk::Label::builder().label("result").build();

        let btn_close = gtk::Button::builder()
            .tooltip_text("close")
            .icon_name("window-close-symbolic")
            .css_classes(vec!["btn", "flat"])
            .build();

        let th = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(5)
            .build();
        th.append(&icon);
        th.append(&label);
        th.append(&btn_close);

        let cbo_sources = self.build_result_sources_dropdown();

        let btn_export = gtk::Button::builder()
            .tooltip_text("Export")
            .icon_name("export")
            .css_classes(vec!["btn", "flat"])
            .build();

        let bar = gtk::ActionBar::builder().hexpand(true).build();
        bar.pack_end(&cbo_sources);
        bar.pack_end(&btn_export);

        let lv = self.build_column_view(result);

        let sw = gtk::ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .child(&lv)
            .build();

        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();

        container.append(&bar);
        container.append(&sw);

        self.nb.append_page(&container, Some(&th));
    }

    fn get_current_statement(&self) -> Option<String> {
        let buffer = self.view.buffer();
        let tm = buffer.get_insert();

        let cursor_iter = buffer.iter_at_mark(&tm);
        // debug!("cursor_iter: {:?}", cursor_iter);

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

        // debug!("text: {} {}", text, trimmed);

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

// impl ActionMap for QueryEditor {}
