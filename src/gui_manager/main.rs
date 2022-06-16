use clap::{Parser};
use libadwaita::{
    gdk::{Display, Monitor},
    gtk::{gdk_pixbuf::Pixbuf, Picture},
    prelude::{
        ApplicationExt, ApplicationExtManual, Cast, DisplayExt, GtkWindowExt, ListModelExt,
        MonitorExt, WidgetExt,
    },
    Application, ApplicationWindow, gio::ApplicationFlags,
};

#[derive(Parser, Debug, Clone)]
#[clap(about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "")]
    image: String,
    #[clap(short, long, value_parser, default_value_t = false)]
    hide_cursor: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let application = Application::new(Some("moe.yummmi.nepnep"), ApplicationFlags::FLAGS_NONE);

    application.connect_activate(move |app| {
        let display = Display::default().expect("No display found");
        let monitors = display.monitors();

        for i in 0..monitors.n_items() {
            let monitor = monitors.item(i).unwrap().dynamic_cast::<Monitor>().unwrap();
            let geometry = monitor.geometry();
            let pixbuf =
                Pixbuf::from_file_at_scale(&args.image, geometry.width(), geometry.height(), true)
                    .unwrap();

            let window = ApplicationWindow::builder()
                .application(app)
                .content(&Picture::for_pixbuf(&pixbuf))
                .build();

            if args.hide_cursor {
                window.set_cursor_from_name(Some("none"));
            }
            window.fullscreen_on_monitor(&monitor);

            window.show();
        }
    });

    application.run_with_args(&vec![""]);
}
