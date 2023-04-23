/**
 * @param {Function} fn
 * @return {Function}
 */
var curry = function (fn) {
  const res = [];
  return function curried(...args) {
    res.push(...args);

    if (res.length == fn.length) {
      return fn(...res);
    } else {
      return curried;
    }
  };
};

/**
 * function sum(a, b) { return a + b; }
 * const csum = curry(sum);
 * csum(1)(2) // 3
 */
