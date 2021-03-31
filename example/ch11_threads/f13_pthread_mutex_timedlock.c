#ifdef __APPLE__

#include <pthread.h>
#include <sys/time.h>
#include <errno.h>
#include <unistd.h>

int pthread_mutex_timedlock(pthread_mutex_t *mutex, const struct timespec *abs_timeout) {
  int result;
  struct timeval current_time;

  do {
    gettimeofday(&current_time, NULL);
    result = pthread_mutex_trylock(mutex);
    if (result == EBUSY) {
      sleep(1);
    } else
      break;
  }
  while (result != 0 && abs_timeout->tv_sec > current_time.tv_sec);

  return result;
}

#endif // __APPLE__
