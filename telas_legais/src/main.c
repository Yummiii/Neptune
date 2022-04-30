#include <adwaita.h>

char *img;
static void activate_cb(GtkApplication *app)
{
  GdkDisplay *display = gdk_display_get_default();
  GListModel *monitors = gdk_display_get_monitors(display);
  int suicidio = g_list_model_get_n_items(monitors);
  //int suicidio = 1;

  for (int i = 0; i < suicidio; i++)
  {
    GtkWidget *window = gtk_application_window_new(app);
    GdkMonitor *monitor = g_list_model_get_item(monitors, i);

    if (img != NULL)
    {
      
      GtkWidget *image = gtk_picture_new_for_filename(img);
      gtk_window_set_child(GTK_WINDOW(window), image);
    }

    gtk_window_fullscreen_on_monitor(GTK_WINDOW(window), monitor);
    gtk_window_set_deletable(GTK_WINDOW(window), false);
    gtk_window_present(GTK_WINDOW(window));
  }
}

int main(int argc, char *argv[])
{
  if (argc > 1)
  {
    img = argv[1];
  }
  else
  {
    img = NULL;
  }
  g_autoptr(AdwApplication) app = NULL;
  app = adw_application_new("moe.yummi.NepNep", G_APPLICATION_FLAGS_NONE);
  g_signal_connect(app, "activate", G_CALLBACK(activate_cb), NULL);
  return g_application_run(G_APPLICATION(app), 0, 0);
}
