/*
 * Other languages here
 *
 */

 /**
  * TS version
  */
// 1. Two Sum
export function twoSum(nums: number[], target: number): number[] {
  const map = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    if (map.has(complement)) {
      return [map.get(complement)!, i];
    }
    map.set(nums[i], i);
  }
  return [];
}

// 2. Contains Duplicate Within Distance
export function containsDuplicatesWithinDistance(nums: number[], distance: number): boolean {
  const map = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    if (map.has(num)) {
      const prevIndex = map.get(num)!;
      if (i - prevIndex <= distance) return true;
    }
    map.set(num, i);
  }
  return false;
}

// 3. Has Nearby Divisible Duplicate
export function hasNearbyDivisibleDuplicate(nums: number[], distance: number, divisor: number): boolean {
  const map = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    if (num % divisor !== 0) continue;
    if (map.has(num)) {
      const prevIndex = map.get(num)!;
      if (i - prevIndex <= distance) return true;
    }
    map.set(num, i);
  }
  return false;
}

// 4. Has Duplicates (HashSet style)
export function hasDuplicates(nums: number[]): boolean {
  const set = new Set<number>();
  for (const num of nums) {
    if (set.has(num)) return true;
    set.add(num);
  }
  return false;
}

// 5. Has Contiguous Subarray Sum Multiple of K
export function hasSubarraySum(nums: number[], k: number): boolean {
  const map = new Map<number, number>();
  map.set(0, -1);
  let sum = 0;

  for (let i = 0; i < nums.length; i++) {
    sum += nums[i];
    const mod = k === 0 ? sum : sum % k;

    if (map.has(mod)) {
      if (i - map.get(mod)! > 1) return true;
    } else {
      map.set(mod, i);
    }
  }

  return false;
}
