📌 DSA Interview Challenge: Sliding Window with Prefix Sum Constraint
Problem Statement:
You are given an array nums that can contain positive, negative, or zero values. You are also given an integer k and a minimum length constraint min_len. Your task is to count all contiguous subarrays of nums that satisfy the following conditions:

The sum of the subarray is within the range [k, 2 * k].

The length of the subarray is at least min_len.

Constraints:

The length of the array nums is at most 100,000.

The elements of nums can be negative, zero, or positive.

k is a positive integer.

min_len is a positive integer.

Examples:
Example 1:
Input: nums = [2, 1, 4, 5, 1], k = 5, min_len = 2
Output: 3
Explanation: The valid subarrays are:

[2, 1, 4] (sum = 7, length = 3)

[1, 4] (sum = 5, length = 2)

[4, 1] (sum = 5, length = 2)

Example 2:
Input: nums = [-1, 2, 3, -2, 4, 1], k = 4, min_len = 2
Output: 4
Explanation: The valid subarrays are:

[2, 3] (sum = 5, length = 2)

[3, -2, 4] (sum = 5, length = 3)

[4, 1] (sum = 5, length = 2)

[3, -2, 4, 1] (sum = 6, length = 4)