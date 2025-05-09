📌 DSA Problem: Subarray Sum with Length Constraint

Problem Statement:

Given an array of integers nums (which can include positive, negative, and zero values) and two integers k (target sum) and min_len (minimum length), count all contiguous subarrays that have a sum equal to k and a length greater than or equal to min_len.

Constraints:

The array can contain positive, negative, and zero values.

The subarray sum must exactly equal k.

The subarray length must be at least min_len.

Example 1:

Input: nums = [1, 2, 3, 4, 2, -1, 3], k = 5, min_len = 2

Output: 4

Explanation: The valid subarrays are [2, 3], [3, 2], [4, 2, -1], and [2, -1, 3].

Example 2:

Input: nums = [5, -2, 7, 3, -2, 1, 2, 3], k = 5, min_len = 3

Output: 3

Explanation: The valid subarrays are [5, -2, 7, -5], [3, -2, 1, 2, 3], and [2, 3].

Example 3:

Input: nums = [-3, 2, 3, 1, 4, -1, 2, 3], k = 5, min_len = 2

Output: 5

Explanation: The valid subarrays are [2, 3], [1, 4], [3, 1, 4, -1, 2], [4, -1, 2, 3], and [2, 3].
