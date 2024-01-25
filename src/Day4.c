#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>
#include <pthread.h>
#include "Timer.h"

#define MAX_LINE_LENGTH 150

typedef struct {
    int *data;
    size_t size;
    size_t capacity;
} Vector;

Vector createVector(size_t capacity) {
    Vector vector;
    vector.data = (int *) malloc(capacity * sizeof(int));
    vector.size = 0;
    vector.capacity = capacity;
    return vector;
}

void popFront(Vector* vector) {
    if (vector->size > 0) {
        memmove(vector->data, vector->data + 1, (vector->size - 1) * sizeof(int));
        vector->size--;

        // Optional: Resize the vector if needed
        if (vector->size < vector->capacity / 2) {
            vector->capacity /= 2;
            vector->data = (int*)realloc(vector->data, vector->capacity * sizeof(int));
            if (vector->data == NULL) {
                perror("Vector data reallocation failed");
                exit(EXIT_FAILURE);
            }
        }
    } else {
        printf("Vector is empty. Cannot pop front.\n");
    }
}

void popBack(Vector* vector) {
    if (vector->size > 0) {
        vector->size--;
        if (vector->size < vector->capacity / 2) {
            vector->capacity /= 2;
            vector->data = (int*)realloc(vector->data, vector->capacity * sizeof(int));
            if (vector->data == NULL) {
                perror("Vector data reallocation failed");
                exit(EXIT_FAILURE);
            }
        }
    } else {
        printf("Vector is empty. Cannot pop back.\n");
    }
}

void pushBack(Vector *vector, int value) {
    if (vector->size == vector->capacity) {
        vector->capacity *= 2;
        vector->data = (int *) realloc(vector->data, vector->capacity * sizeof(int));
    }
    vector->data[vector->size++] = value;
}

void printVector(Vector *vector) {
    printf("Vector: ");
    for (size_t i = 0; i < vector->size; i++) {
        printf("%d ", vector->data[i]);
    }
    printf("\n");
}

void freeVector(Vector *vector) {
    free(vector->data);
    vector->size = 0;
    vector->capacity = 0;
}

int calculate_score(Vector win, Vector giv);

int main() {
    timer_init();
    Timer timer = timer_new();
    timer_start(&timer);

    char line[MAX_LINE_LENGTH];
    int result_part_2 = 0;

    FILE *file = fopen("../day4_input.txt", "r");

    if (file == NULL) {
        perror("Error opening file");
        return 1;
    }

    char *semi_colon;
    Vector queue = createVector(1);
    Vector d = createVector(1);
    while(fgets(line,sizeof(line),file) != NULL){
        //Get Card ID number
        int cardID = 0;
        semi_colon = strchr(line, ':');
        int sc_index = semi_colon - line;
        int sum = 0,counter = 0;
        Vector winning_numbers = createVector(10);
        Vector given_numbers = createVector(25);
        for (int i = sc_index; 0<i; i--) {
            if (isdigit(line[i])) {
                int digit = line[i] - '0';
                sum += digit * pow(10, counter);
                counter++;
            } else if (isdigit(line[i + 1])) {
                cardID = sum;
                sum = 0;
                counter = 0;
            }
        }

        char *vert_line;
        vert_line = strchr(line, '|');
        int index = vert_line - line;

        //get length of a line
        int lineLength = 0;
        while (line[lineLength] != '\0') {
            lineLength++;
        }

        for (int i = index; line[i] != ':'; i--) {
            if (isdigit(line[i])) {
                int digit = line[i] - '0';
                sum += digit * pow(10, counter);
                counter++;
            } else if (isdigit(line[i + 1])) {
                pushBack(&winning_numbers, sum);
                sum = 0;
                counter = 0;
            }
        }

        for (int i = lineLength; line[i] != '|'; i--) {
            if (isdigit(line[i])) {
                int digit = line[i] - '0';
                sum += digit * pow(10, counter);
                counter++;
            } else if (isdigit(line[i + 1])) {
                pushBack(&given_numbers, sum);
                sum = 0;
                counter = 0;
            }
        }

        int matches = 0;
        for (int i = 0; i < winning_numbers.size; i++) {
            for (int j = 0; j < given_numbers.size; j++) {
                if (winning_numbers.data[i] == given_numbers.data[j]) {
                    matches++;
                }
            }
        }
        //printf("%s",line);
        pushBack(&d,matches);
        pushBack(&queue,cardID);

    }

    while (queue.size != 0) {
        result_part_2++;
        int k = queue.data[0];
        popFront(&queue);
        for (int i = k + 1; i <= k + d.data[k-1]; i++) {
            pushBack(&queue, i);
        }
    }

    printf("%d",result_part_2);
    timer_stop(&timer);
    timer_print("\nTime elapsed: ",timer);
}


int part_1(void){
    int result_part_1 = 0;
    timer_init();
    Timer timer = timer_new();
    timer_start(&timer);

    FILE *file = fopen("../day4_input.txt", "r");

    if (file == NULL) {
        perror("Error opening file");
        return 1;
    }

    char line[MAX_LINE_LENGTH];
    char *vert_line;

    int counter = 0, sum = 0;

    while (fgets(line, sizeof(line), file) != NULL) {
        Vector winning_numbers = createVector(10);
        Vector given_numbers = createVector(25);
        vert_line = strchr(line, '|');
        int index = vert_line - line;

        //get length of a line
        int lineLength = 0;
        while (line[lineLength] != '\0') {
            lineLength++;
        }

        for (int i = index; line[i] != ':'; i--) {
            if (isdigit(line[i])) {
                int digit = line[i] - '0';
                sum += digit * pow(10, counter);
                counter++;
            } else if (isdigit(line[i + 1])) {
                pushBack(&winning_numbers, sum);
                sum = 0;
                counter = 0;
            }
        }

        for (int i = lineLength; line[i] != '|'; i--) {
            if (isdigit(line[i])) {
                int digit = line[i] - '0';
                sum += digit * pow(10, counter);
                counter++;
            } else if (isdigit(line[i + 1])) {
                pushBack(&given_numbers, sum);
                sum = 0;
                counter = 0;
            }
        }

        printf("%d",*winning_numbers.data);
        result_part_1 += calculate_score(winning_numbers, given_numbers);

        printVector(&winning_numbers);
        printVector(&given_numbers);
        freeVector(&winning_numbers);
        freeVector(&given_numbers);
    }

    printf("Result part 1: %d", result_part_1);

    fclose(file);

    timer_stop(&timer);
    timer_print("\nTimer elapsed: ", timer);
}

int calculate_score(Vector win, Vector giv) {

    int power = -1, score = 0;
    for (int i = 0; i < win.size; i++) {
        for (int j = 0; j < giv.size; j++) {
            if (win.data[i] == giv.data[j]) {
                power++;
            }
        }
    }
    score = pow(2, power);
    return score;
}