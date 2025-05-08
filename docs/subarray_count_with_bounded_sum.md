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


======================

Best way to answer


💡 Interviewee Response: Thought Process, Time, and Space Complexity
Approach:
To efficiently find all the valid subarrays, a naive O(n^2) approach that checks every possible subarray would be too slow. Instead, I will use the sliding window technique with a running prefix sum to maintain a moving sum of the subarray as I expand and shrink the window.

Steps:

Use a sliding window approach where start is the left boundary and end is the right boundary of the current subarray.

Add each new number to the prefix_sum as you expand the window.

Shrink the window from the left if the prefix_sum exceeds 2 * k to keep the sum within the valid range.

Once the prefix_sum is within [k, 2 * k] and the window length is at least min_len, count all valid subarrays ending at the current end index.

Repeat this process until the entire array is covered.

Why Sliding Window?

The sliding window approach ensures each element is added and removed from the running sum at most once, resulting in an O(n) time complexity.

This approach minimizes the need for nested loops, avoiding the quadratic time penalty of brute force.

Time Complexity:

O(n): The start pointer only moves forward, never backward, and each element is processed once as the end pointer moves from left to right.

Space Complexity:

O(1): The space used is constant because only a few extra variables (prefix_sum, count, start) are used, regardless of the size of the input array.