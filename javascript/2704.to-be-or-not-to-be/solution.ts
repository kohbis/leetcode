type ToBeOrNotToBe = {
  toBe: (val: any) => boolean;
  notToBe: (val: any) => boolean;
};

function expect(val: any): ToBeOrNotToBe {
  const toBe = (x: any): boolean => {
    if (val !== x || typeof val !== typeof x) {
      throw new Error("Not Equal");
    }
    return true;
  };

  const notToBe = (x: any): boolean => {
    if (val == x && typeof val == typeof x) {
      throw new Error("Equal");
    }
    return true;
  };

  return {
    toBe,
    notToBe,
  };
}

/**
 * expect(5).toBe(5); // true
 * expect(5).notToBe(5); // throws "Equal"
 */
