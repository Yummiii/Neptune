#include <adwaita.h>
#include <stdio.h>

char *img;
GList *telas = NULL;
static void activate_cb(GtkApplication *app)
{
  GdkDisplay *display = gdk_display_get_default();
  GListModel *monitors = gdk_display_get_monitors(display);
  int suicidio = g_list_model_get_n_items(monitors);
  // int suicidio = 1;

  for (int i = 0; i < suicidio; i++)
  {
    GtkWidget *window = adw_application_window_new(app);
    gtk_window_set_destroy_with_parent(GTK_WINDOW(window), true);
    gtk_window_set_default_size(GTK_WINDOW(window), 545, 735);

    GdkMonitor *monitor = (GdkMonitor *)g_list_model_get_item(monitors, i);

    if (img != NULL)
    {
      GdkPixbuf *img_buf = gdk_pixbuf_new_from_file_at_scale(img, 1920, 1080, true, NULL);
      GtkWidget *image = gtk_picture_new_for_pixbuf(img_buf);
      // GtkWidget *image = gtk_picture_new_for_filename(img);
      // gtk_picture_set_can_shrink(GTK_PICTURE(image), true);
      adw_application_window_set_content(ADW_APPLICATION_WINDOW(window), image);
    }

    gtk_window_fullscreen_on_monitor(GTK_WINDOW(window), monitor);
    gtk_window_set_deletable(GTK_WINDOW(window), false);
    telas = g_list_append(telas, GTK_WINDOW(window));
    gtk_window_present(GTK_WINDOW(window));
  }
}

void top_nep(char *path)
{
  printf("Top Nep: [%s]\n", path);
  if (path != NULL)
  {
    img = path;
  }
  g_autoptr(AdwApplication) app = NULL;
  app = adw_application_new("moe.yummi.nepnep", G_APPLICATION_NON_UNIQUE);
  g_signal_connect(app, "activate", G_CALLBACK(activate_cb), NULL);
  g_application_run(G_APPLICATION(app), 0, 0);
}

void down_nep()
{
  printf("Down Nep\n");

  for (int i = 0; i < (int)g_list_length(telas); i++)
  {
    GtkWidget *window = g_list_nth_data(telas, i);
    gtk_window_destroy(GTK_WINDOW(window));
  }

  g_list_free(telas);
  telas = NULL;
}