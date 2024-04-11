#include <gtk/gtk.h>
#include <mongoc/mongoc.h>

static void activate(GtkApplication* app, gpointer user_data) {
    GtkWidget* window;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW (window), "Hello");
    gtk_window_set_default_size(GTK_WINDOW (window), 200, 200);

    gtk_window_present(GTK_WINDOW (window));
}

int main(int argc, char** argv) {
    GtkApplication *app;
    int status;

    mongoc_client_t* client;
    mongoc_collection_t* collection;
    mongoc_cursor_t* cursor;

    const bson_t* doc;
    bson_t* query;
    char* str;

    mongoc_init();
    client = mongoc_client_new("mongodb://root:root@localhost/db?authSource=admin");
    collection = mongoc_client_get_collection(client, "blogDB", "users");

    query = bson_new();
    cursor = mongoc_collection_find_with_opts(collection, query, NULL, NULL);

    while (mongoc_cursor_next(cursor, &doc)) {
        str = bson_as_canonical_extended_json(doc, NULL);
        printf("%s\n", str);
        bson_free(str);
    }

    bson_destroy(query);
    mongoc_cursor_destroy(cursor);
    mongoc_collection_destroy(collection);
    mongoc_client_destroy(client);
    mongoc_cleanup();

    app = gtk_application_new("org.gtk.example", G_APPLICATION_DEFAULT_FLAGS);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);

    status = g_application_run(G_APPLICATION (app), argc, argv);
    g_object_unref(app);

    return status;
}
