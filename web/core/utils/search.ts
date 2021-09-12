// see
// https://github.com/numpy/numpy/blob/623bc1fae1d47df24e7f1e29321d0c0ba2771ce0/numpy/core/src/multiarray/compiled_base.c

const LIKELY_IN_CACHE_SIZE = 8;

export const linear_search = (key: number, arr: number[], i0: number):
    number => {
      let i;
      for (i = i0; i < arr.length && key >= arr[i]; i++)
        ;
      return i - 1;
    }

export const binary_search_with_guess =
    (key: number, arr: number[], guess: number): number => {
      let imin = 0;
      let imax = arr.length;

      // Handle keys outside of the arr range first
      if (key > arr[-1]) {
        return arr.length;
      } else if (key < arr[0]) {
        return -1;
      }

      // if len <= 4 use linear search.
      // from above we know key >= arr[0] when we start.
      if (arr.length <= 4) {
        return linear_search(key, arr, 1);
      }
      if (guess > arr.length - 3) {
        guess = arr.length - 3;
      }
      if (guess < 1) {
        guess = 1;
      }

      // check most likely values: guess - 1, guess, guess + 1
      if (key < arr[guess]) {
        if (key < arr[guess - 1]) {
          imax = guess - 1;
          // last attempt to restrict search to items in cache
          if (guess > LIKELY_IN_CACHE_SIZE &&
              key >= arr[guess - LIKELY_IN_CACHE_SIZE]) {
            imin = guess - LIKELY_IN_CACHE_SIZE;
          }
        } else {
          // key >= arr[guess - 1]
          return guess - 1;
        }
      }

      else {
        // key >= arr[guess]
        if (key < arr[guess + 1]) {
          return guess;
        } else {
          // key >= arr[guess + 1]
          if (key < arr[guess + 2]) {
            return guess + 1;
          } else {
            // key >= arr[guess + 2]
            imin = guess + 2;
            // last attempt to restrict search to items in cache
            if (guess < arr.length - LIKELY_IN_CACHE_SIZE - 1 &&
                key < arr[guess + LIKELY_IN_CACHE_SIZE]) {
              imax = guess + LIKELY_IN_CACHE_SIZE;
            }
          }
        }
      }

      // finally, find index by bisection
      while (imin < imax) {
        const imid = imin + ((imax - imin) >> 1);
        if (key >= arr[imid]) {
          imin = imid + 1;
        } else {
          imax = imid;
        }
      }
      return imin - 1;
    }
