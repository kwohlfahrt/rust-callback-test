#include <stddef.h>

static void * user_data;
static int (* callback)(int, void *);

void set_handler(int (* p_callback)(int, void *), void * p_user_data) {
    user_data = p_user_data;
    callback = p_callback;
}

void * unset_handler() {
    callback = NULL;
    void * p_user_data = user_data;
    user_data = NULL;
    return p_user_data;
}

int invoke_handler(int x) {
    if (callback != NULL)
        return callback(x, user_data);
    else
        return 1234;
}
