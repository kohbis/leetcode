/**
 * @param {number[]} arr
 * @param {Function} fn
 * @return {number[]}
 */
var filter = function (arr, fn) {
  let filterd = [];
  for (let i = 0; i < arr.length; i++) {
    if (fn(arr[i], i)) {
      filterd.push(arr[i]);
    }
  }
  return filterd;
};
