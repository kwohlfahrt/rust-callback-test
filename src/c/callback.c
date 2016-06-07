#include <stddef.h>
#include <stdlib.h>

struct Callback {
    void * user_data;
    int (* callback)(int, void*);
    const struct Callback * prev;
    const struct Callback * next;
};
const struct Callback * callbacks = NULL;

const struct Callback * add_callback(int (* p_callback)(int, void *), void * p_user_data) {
    struct Callback * callback = malloc(sizeof(struct Callback));
    if (callback == NULL)
        return NULL;

    callback->user_data = p_user_data;
    callback->callback = p_callback;
    callback->next = NULL;
    callback->prev = NULL;

    if (callbacks == NULL) {
        callbacks = callback;
    } else {
        const struct Callback * last = callbacks;
        while (last->next != NULL)
            last = last->next;
        ((struct Callback *) last)->next = callback;
        callback->prev = last;
    }
    return callback;
}

void * remove_callback(struct Callback * callback) {
    if (callback->prev) {
        ((struct Callback * ) callback->prev)->next = callback->next;
    } else {
        callbacks = callback->next;
    }
    if (callback->next) {
        ((struct Callback * ) callback->next)->prev = callback->prev;
    }
    void * ret = callback->user_data;
    free(callback);
    return ret;
}

int invoke_callbacks(int x) {
    const struct Callback * callback = callbacks;
    int ret = 0;
    while (callback != NULL) {
        ret += (callback->callback)(x, callback->user_data);
        callback = callback->next;
    };
    return ret;
}
