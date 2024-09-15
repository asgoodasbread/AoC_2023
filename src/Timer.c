#include "Timer.h"

#if defined(__linux)

void timer_init(void) {
    // Nothing to init here
}

Timer timer_new(void) {
    Timer timer;
    clock_gettime(CLOCK_REALTIME, &timer.time);
    return timer;
}

void timer_start(Timer *timer) {
    clock_gettime(CLOCK_REALTIME, &timer->time);
}

void timer_stop(Timer *timer) {
    static struct timespec now;

    clock_gettime(CLOCK_REALTIME, &now);
    now.tv_sec -= timer->time.tv_sec;
    now.tv_nsec -= timer->time.tv_nsec;
    timer->time = now;
}

u64 timer_elapsed(const Timer timer) {
    return timer.time.tv_sec * 1e9 + timer.time.tv_nsec;
}

#elif defined(_WIN32)

static LARGE_INTEGER freq;
void timer_init(void) {
    QueryPerformanceFrequency(&freq);
}

Timer timer_new(void) {
    Timer timer;
    QueryPerformanceCounter(&timer.time);
    return timer;
}

void timer_start(Timer *timer) {
    QueryPerformanceCounter(&timer->time);
}

void timer_stop(Timer *timer) {
    static LARGE_INTEGER now;

    QueryPerformanceCounter(&now);
    now.QuadPart -= timer->time.QuadPart;

    now.QuadPart *= 1e9;
    now.QuadPart /= freq.QuadPart;
    timer->time = now;
}

unsigned long timer_elapsed(const Timer timer) {
    return (unsigned long)timer.time.QuadPart;
}

#else // Other platforms, lower precision

void timer_init(void) {
    // Nothing to init here
}

Timer timer_new(void) {
    Timer timer;
    timer.time = clock();
    return timer;
}

void timer_start(Timer *timer) {
    *timer.time = clock();
}

void timer_stop(Timer *timer) {
    timer->time = clock() - timer->time;
}

u64 timer_elapsed(const Timer timer) {
    return (u64)timer.time * (1e9 / CLOCKS_PER_SEC);
}

#endif

void timer_print(const char* prompt, Timer timer) {
    unsigned long elapsed = timer_elapsed(timer);

    printf("%s", prompt);
    if (elapsed / (unsigned long)1e9) {
        printf("%.3lf s\n", (double)elapsed / 1e9);
    } else if (elapsed / (unsigned long)1e6) {
        printf("%.3lf ms\n", (double)elapsed / 1e6);
    } else if (elapsed / (unsigned long)1e3) {
        printf("%.3lf μs\n", (double)elapsed / 1e3);
    } else {
        printf("%lu ns\n", elapsed);
    }
}

// void print_benchmark(const char* prompt, u64 elapsed) {
//     printf("%s", prompt);
//     if (elapsed / (u64)1e9) {
//         printf("%.3lf s\n", (double)elapsed / 1e9);
//     } else if (elapsed / (u64)1e6) {
//         printf("%.3lf ms\n", (double)elapsed / 1e6);
//     } else if (elapsed / (u64)1e3) {
//         printf("%.3lf μs\n", (double)elapsed / 1e3);
//     } else {
//         printf("%lu ns\n", elapsed);
//     }
// }