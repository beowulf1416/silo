use tracing::{debug, error};

use gtk::{Application, ApplicationWindow, gio, glib, prelude::*};

use crate::APP_TITLE;

#[derive(Debug, Clone)]
pub enum MainWindowInputMessage {
    CloseRequested,
}

pub struct MainWindow {
    app: gtk::Application,
}

impl MainWindow {
    pub fn build(app: &Application) {
        // main event loop thread
        let (sender, receiver) = async_channel::unbounded::<MainWindowInputMessage>();

        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_TITLE)
            .default_width(800)
            .default_height(600)
            .build();

        window.connect_close_request(move |_| {
            if let Err(e) = sender.try_send(MainWindowInputMessage::CloseRequested) {
                error!("unable to send message: {:?}", e);
            }
            return glib::Propagation::Stop;
        });

        let mw = Self { app: app.clone() };

        // setup receiver
        glib::MainContext::default().spawn_local(async move {
            while let Ok(msg) = receiver.recv().await {
                mw.process_message(msg);
            }
        });

        window.present();
    }

    fn process_message(&self, msg: MainWindowInputMessage) {
        match msg {
            MainWindowInputMessage::CloseRequested => {
                debug!("close requested");
                self.app.quit();
            }
        }
    }
}
