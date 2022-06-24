use libadwaita::{
    gdk::{Display, Monitor},
    gio::ApplicationFlags,
    gtk::{gdk_pixbuf::Pixbuf, Picture},
    prelude::{
        ApplicationExt, ApplicationExtManual, Cast, DisplayExt, GtkWindowExt, ListModelExt,
        MonitorExt, WidgetExt,
    },
    traits::AdwApplicationWindowExt,
    Application, ApplicationWindow,
};

pub fn open_block_gui( image: Option<String>, show_cursor: bool) {
    let application = Application::new(Some("moe.yummmi.Neptune"), ApplicationFlags::FLAGS_NONE);

    application.connect_activate(move |app| {
        let display = Display::default().expect("No display found");
        let monitors = display.monitors();

        for i in 0..monitors.n_items() {
            let monitor = monitors.item(i).unwrap().dynamic_cast::<Monitor>().unwrap();
            let window = ApplicationWindow::new(app);
            window.fullscreen_on_monitor(&monitor);

            if let Some(img) = &image {
                let geometry = monitor.geometry();
                let pixbuf = Pixbuf::from_file_at_scale(img, geometry.width(), geometry.height(), true).unwrap();
                window.set_content(Some(&Picture::for_pixbuf(&pixbuf)))
            }

            if !show_cursor {
                window.set_cursor_from_name(Some("none"));
            }

            window.show();
        }
    });

    application.run_with_args(&vec![""]);
}
