#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>

enum MachineMove {
    Left = -1,
    Stay = 0,
    Right = 1
};

struct MachineTransition {
    int writeValue;
    enum MachineMove move;
    int switchTo;
};

struct MachineState {
    struct MachineTransition zero;
    struct MachineTransition one;
};

struct MachineDefinition {
    struct MachineState *states;
};

struct Tape {
    int* data;
    int count;
    int size;
};

struct Machine {
    struct MachineDefinition *definition;
    int state;
    int position;
    struct Tape left;
    struct Tape right;
};

void dump(struct Machine *self, char* buffer, int bufferSize) {
    int written = snprintf(buffer, bufferSize, "s:%i p:%i ", self->state, self->position);
    int position = 0;
    while(written < bufferSize-1 && position < self->left.count) {
        buffer[written++] = self->left.data[self->left.count-position++-1] ? '1' : '0';
    }
    if(written < bufferSize-1)
        buffer[written++] = '|';
    position = 0;
    while(written < bufferSize-1 && position < self->right.count) {
        buffer[written++] = self->right.data[position++] ? '1' : '0';
    }
    buffer[written] = 0;
}

void step(struct Machine* self) {
    if(self->state < 0) {
        return;
    }
    int index;
    struct Tape *pTape;
    if(self->position >= 0) {
        index = self->position;
        pTape = &(self->right);
    } else {
        index = -self->position - 1;
        pTape = &(self->left);
    }
    if(index >= pTape->size) {
        pTape->size = pTape->size == 0 ? 1 : pTape->size << 1;
        pTape->data = realloc(pTape->data, pTape->size * sizeof(int));
    }
    if(index >= pTape->count) {
        pTape->count++;
        pTape->data[index] = 0;
    }
    int read = pTape->data[index];
    struct MachineState *state = self->definition->states + self->state;
    struct MachineTransition *transition = read==1 ? &state->one : &state->zero;
    pTape->data[index] = transition->writeValue;
    self->state = transition->switchTo;
    self->position += transition->move;
}

int main(int argc, char** argv) {
    struct MachineDefinition def = (struct MachineDefinition) {
        (struct MachineState[]) {
            (struct MachineState) {
                (struct MachineTransition) { 1, Right, 1 },
                (struct MachineTransition) { 1, Left, 2 }},
            (struct MachineState) {
                (struct MachineTransition) { 1, Right, 2 },
                (struct MachineTransition) { 1, Right, 1 }},
            (struct MachineState) {
                (struct MachineTransition) { 1, Right, 3 },
                (struct MachineTransition) { 0, Left, 4 }},
            (struct MachineState) {
                (struct MachineTransition) { 1, Left, 0 },
                (struct MachineTransition) { 1, Left, 3 }},
            (struct MachineState) {
                (struct MachineTransition) { 1, Right, -1 },
                (struct MachineTransition) { 0, Left, 0 }}
        }
    };
    for(int j = 0; j<10; j++) {
        int i = 0;
        struct timeval before;
        struct timeval after;
        struct Machine machine = (struct Machine) { &def, 0, 0, {0, 0, 0}, { 0, 0, 0}};
        gettimeofday(&before, NULL);
        while(machine.state != -1) {
            step(&machine);
            i+=1;
        }
        gettimeofday(&after, NULL);
        printf("i:%d %ldms\n", i, (after.tv_sec-before.tv_sec)*1000 + (after.tv_usec - before.tv_usec)/1000);
    }
}
