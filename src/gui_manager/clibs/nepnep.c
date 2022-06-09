#include <adwaita.h>
#include <stdio.h>

char *img;

static void activate_cb(GtkApplication *app)
{
  GdkDisplay *display = gdk_display_get_default();
  GListModel *monitors = gdk_display_get_monitors(display);
  int suicidio = g_list_model_get_n_items(monitors);;

  for (int i = 0; i < suicidio; i++)
  {
    GtkWidget *window = adw_application_window_new(app);
    gtk_window_set_destroy_with_parent(GTK_WINDOW(window), true);
    gtk_window_set_default_size(GTK_WINDOW(window), 545, 735);

    GdkMonitor *monitor = (GdkMonitor *)g_list_model_get_item(monitors, i);

    if (img != NULL)
    {
      GdkRectangle geometry;
      gdk_monitor_get_geometry(monitor, &geometry);
      GdkPixbuf *img_buf = gdk_pixbuf_new_from_file_at_scale(img, geometry.width, geometry.height, true, NULL);
      GtkWidget *image = gtk_picture_new_for_pixbuf(img_buf);
      adw_application_window_set_content(ADW_APPLICATION_WINDOW(window), image);
    }

    gtk_window_fullscreen_on_monitor(GTK_WINDOW(window), monitor);
    gtk_window_set_deletable(GTK_WINDOW(window), true);
    gtk_window_present(GTK_WINDOW(window));
  }
}

void top_nep(char *path)
{
  g_print("\nTop Nep: [%s]\n", path);
  img = path;
  AdwApplication *app = adw_application_new("moe.yummi.nepnep", G_APPLICATION_NON_UNIQUE);
  g_signal_connect(app, "activate", G_CALLBACK(activate_cb), NULL);
  g_application_run(G_APPLICATION(app), 0, 0);
  g_object_unref(app);
}