/**
 * @param {Array} arr
 * @param {number} size
 * @return {Array[]}
 */
const chunk = function (arr, size) {
  const chunked = [];
  for (i = 0; i < arr.length; i += size) {
    chunked.push(arr.slice(i, i + size));
  }
  return chunked;
};
