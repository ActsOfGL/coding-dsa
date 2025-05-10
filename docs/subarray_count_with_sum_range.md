# 📌 DSA Problem: Count Subarrays Within Sum Range and Length Constraint

### Problem:
Given an array of integers nums, which can include positive, negative, or zero values, and two additional integers, k and min_len, you need to count all the contiguous subarrays whose sum is between k and 2 * k (inclusive) and whose length is at least min_len.

### Constraints:
The sum of the subarray must be between k and 2 * k (inclusive).
The subarray must have a length of at least min_len.
The function should aim for an efficient O(n) solution using a sliding window approach.

Example 1:
Input: nums = [1, 2, -1, 3, 4], k = 3, min_len = 2
Output: 4
Explanation:
The valid subarrays are:
  1. [1, 2] (sum = 3, length = 2)
  2. [1, 2, -1, 3] (sum = 5, length = 4)
  3. [2, -1, 3] (sum = 4, length = 3)
  4. [3, 4] (sum = 7, length = 2)

Example 2:
Input: nums = [5, -1, 3, -2, 6], k = 4, min_len = 3
Output: 2
Explanation:
The valid subarrays are:
  1. [5, -1, 3] (sum = 7, length = 3)
  2. [-1, 3, -2, 6] (sum = 6, length = 4)



============================


# 💡 Interviewee Response (Thinking Aloud):

### Approach:
I'll use a sliding window approach because I need to find contiguous subarrays with a bounded sum range.
I need to carefully expand and contract the window to ensure the sum remains within the target range.
This approach should naturally handle negative numbers, which can complicate prefix sum methods.

### Edge Cases:
If the array is empty, return 0.
If min_len is larger than the array length, return 0, as no subarray can satisfy the length constraint.

### Space and Time Complexity:
Time: O(n), as each element is added and removed from the window at most once.
Space: O(1) extra space if we ignore the input array.

### Final Steps:
Expand the window by moving the right pointer (end).
Shrink the window as needed to restore validity based on the sum range constraint.
Count the subarrays only when the window meets the length requirement.