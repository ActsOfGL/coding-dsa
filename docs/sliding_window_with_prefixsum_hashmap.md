# ✅ Question: Subarray Count with Zeros and Negatives Allowed

## 🧠 Description:
Given an array that may contain zeros or negative values, count how many contiguous subarrays have a sum equal to k.

## 💡 Insight:
We can’t use a shrinking window like before, because the sum can shrink or grow unpredictably. Instead, use a prefix sum + hashmap approach. If the difference between the current prefix sum and k has been seen before, the number of such occurrences equals the number of valid subarrays ending at the current index.

=========================================================================

# 🎯 Curveball Challenge: Suspicious Login Pattern Detection
Problem Statement
You’re given a list of login durations in minutes for a user during a single day:

```
let logs = vec![u32]; // all values are positive, could be large
```

Your goal is to determine how many contiguous slices (subarrays) of login durations exist such that the total login time equals exactly target_minutes.

But here’s the catch:
 - The login durations might contain zeros or repeated numbers.
 - The array might be very long.
 - Multiple subarrays might overlap.

Constraints:
 - logs.len() <= 100_000
 - target_minutes is a positive u32

📌 Input
```
let logs = vec![3, 4, 7, 2, -3, 1, 4, 2];
let target_minutes = 7;
```

✅ Output
```
// Returns 4, because these subarrays sum to 7:
// [3,4], [7], [4,2,1], [1,4,2]
```
Notes:
- Subarrays must be contiguous.
- You must aim for O(n) time and O(n) space.
- Sliding window alone won’t work because of negatives and zeros.
- You must decide when to use prefix sum and when not to.
