
export const indexed = (arr: number[]): {
  idx: number,
  value: number
}[] => { return arr.map((value, idx) => { return {idx, value}; })}

export const indexedSort =
    (arr: number[], descending = false): {idx: number, value: number}[] => {
      return arr.map((value, idx) => { return {idx, value}; }).sort((a, b) => {
        let ans = 0;
        if (a.value < b.value)
          ans = -1;
        if (a.value > b.value)
          ans = 1;
        if (descending)
          ans *= -1;
        return ans
      });
    }
