type ReturnObj = {
  increment: () => number;
  decrement: () => number;
  reset: () => number;
};

function createCounter(init: number): ReturnObj {
  let count = init;

  const increment = (): number => {
    return ++count;
  };

  const decrement = (): number => {
    return --count;
  };

  const reset = (): number => {
    count = init;
    return count;
  };

  return { increment, decrement, reset };
}

/**
 * const counter = createCounter(5)
 * counter.increment(); // 6
 * counter.reset(); // 5
 * counter.decrement(); // 4
 */
