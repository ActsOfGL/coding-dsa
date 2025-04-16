/*
 * Other languages here
 *
 */

 /**
  * TS version
  */
  // two sum function in TypeScript
  function twoSum(nums: number[], target: number): [number, number] | null {
    const map = new Map<number, number>();
  
    for (let i = 0; i < nums.length; i++) {
      const complement = target - nums[i];
      if (map.has(complement)) {
        return [map.get(complement)!, i];
      }
      map.set(nums[i], i);
    }
  
    return null;
  }