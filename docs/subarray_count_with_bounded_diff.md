📌 DSA Interview Challenge: Subarrays with Bounded Max-Min Difference

Problem Statement:Given an array nums that may contain positive, negative, or zero values, along with two parameters k and min_len, your task is to count all contiguous subarrays where:

The difference between the maximum and minimum element is exactly k.

The length of the subarray is at least min_len.

Constraints:

The length of the array nums is at most 100,000.

The elements of nums can be negative, zero, or positive.

k and min_len are positive integers.

Examples:Example 1:Input: nums = [2, 1, 4, 5, 1], k = 3, min_len = 2Output: 2Explanation: The valid subarrays are:

[2, 1, 4] (max - min = 4 - 1 = 3, length = 3)

[4, 1] (max - min = 4 - 1 = 3, length = 2)

Example 2:Input: nums = [1, 5, 2, 4, 3], k = 2, min_len = 3Output: 1Explanation: The only valid subarray is [5, 2, 4] (max - min = 5 - 2 = 3, length = 3)
