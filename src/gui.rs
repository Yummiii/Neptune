use chrono::Utc;
use libadwaita::{
    gdk::{Display, Monitor},
    gio::{ApplicationFlags, File},
    gtk::{gdk_pixbuf::Pixbuf, Picture},
    prelude::{
        ApplicationExt, ApplicationExtManual, Cast, DisplayExt, GtkWindowExt, ListModelExt,
        MonitorExt, WidgetExt,
    },
    traits::AdwApplicationWindowExt,
    Application, ApplicationWindow,
};
use std::path::Path;

pub fn open_gui(image: Option<String>, hide_cursor: bool, windowed: bool, title: Option<String>) {
    let application = Application::new(Some(&format!("moe.yummmi.Neptune_{}", Utc::now().timestamp_millis())), ApplicationFlags::FLAGS_NONE);

    application.connect_activate(move |app| {
        let display = Display::default().expect("No display found");
        let monitors = display.monitors();

        for i in 0..if windowed { 1 } else { monitors.n_items() } {
            let monitor = monitors.item(i).unwrap().dynamic_cast::<Monitor>().unwrap();
            let window = ApplicationWindow::new(app);
            if !windowed {
                window.fullscreen_on_monitor(&monitor);
            }

            if let Some(img) = &image {
                if img != "" {
                    if Path::new(img).exists() {
                        let geometry = monitor.geometry();
                        let pixbuf = Pixbuf::from_file_at_scale(
                            img,
                            geometry.width(),
                            geometry.height(),
                            true,
                        )
                        .unwrap();
                        window.set_content(Some(&Picture::for_pixbuf(&pixbuf)))
                    } else {
                        window.set_content(Some(&Picture::for_file(&File::for_uri(&img))))
                    }
                }
            }

            if hide_cursor {
                window.set_cursor_from_name(Some("none"));
            }

            window.set_title(Some(&title.as_ref().unwrap_or(&"NepNep".to_string())));
            window.show();
        }
    });

    application.run_with_args(&vec![""]);
}
