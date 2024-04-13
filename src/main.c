#include <gtk/gtk.h>
#include <mongoc/mongoc.h>

typedef struct {
	GtkWidget *container;
} Comments;

static void append_entries_on_clicked(GtkButton *button, gpointer data) {
	Comments *comments = (Comments*) data;

	mongoc_client_t *client;
    mongoc_collection_t *collection;
    mongoc_cursor_t *cursor;

    const bson_t *doc;
	const bson_value_t *value;
    bson_t *query;

    mongoc_init();
    client = mongoc_client_new("mongodb://root:root@localhost/db?authSource=admin");
    collection = mongoc_client_get_collection(client, "blogDB", "entries");

    query = bson_new();
    cursor = mongoc_collection_find_with_opts(collection, query, NULL, NULL);

    while (mongoc_cursor_next(cursor, &doc)) {
		bson_iter_t iter;
		GtkWidget *tv = gtk_text_view_new();
		GtkTextBuffer *tb = gtk_text_view_get_buffer(GTK_TEXT_VIEW(tv));
		GString *text = g_string_new("");

		if (bson_iter_init(&iter, doc)) {
			while (bson_iter_next(&iter)) {
				if (g_strcmp0(bson_iter_key(&iter), "_id")) {
					g_print("Found element key: \"%s\"\n", bson_iter_key(&iter));
					value = bson_iter_value(&iter);
					g_print("got it %u\n", value->value_type);
					if (value->value_type == 2) {
						text = g_string_append(text, value->value.v_utf8.str);
					}
				}
			}
		}
		gtk_text_buffer_set_text(tb, text->str, -1);
		gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(tv), GTK_WRAP_WORD_CHAR);
		gtk_box_append((GtkBox*) comments->container, tv);
    }

    bson_destroy(query);
    mongoc_cursor_destroy(cursor);
    mongoc_collection_destroy(collection);
    mongoc_client_destroy(client);
    mongoc_cleanup();
}

static void activate(GtkApplication *app, gpointer user_data) {
    GtkWidget *window;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW (window), "Hello");
    gtk_window_set_default_size(GTK_WINDOW (window), 400, 400);

	GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 10);
	gtk_box_set_homogeneous((GtkBox*)box, FALSE);

	GtkWidget *button = gtk_button_new_with_label("fetch");
	gtk_button_set_has_frame((GtkButton*)button, TRUE);
    gtk_widget_set_margin_start(button, 300);
    gtk_widget_set_margin_end(button, 300);

	gtk_box_append((GtkBox*)box, button);

	static Comments comments;
	Comments *comments_ptr = &comments;
	comments_ptr->container = box;

	g_print("hello\n");
	g_signal_connect((GtkButton*)button, "clicked", G_CALLBACK (append_entries_on_clicked), comments_ptr);

	gtk_window_set_child(GTK_WINDOW (window), box);
    gtk_window_present(GTK_WINDOW (window));
}

int main(int argc, char **argv) {
    GtkApplication *app;
    int status;

    app = gtk_application_new("com.github.aichingert.glongo", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);

    status = g_application_run(G_APPLICATION (app), argc, argv);
    g_object_unref(app);

    return status;
}
