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

💡 Interviewee Response: Thought Process, Time, and Space Complexity

Approach:To efficiently count all valid subarrays, I'll use a sliding window approach with two monotonic deques to track the maximum and minimum elements within the current subarray. This allows us to adjust the window dynamically as we move through the array, maintaining the required constraints without recalculating the max and min on every step.

Steps:

Use two deques, one for tracking the maximum values and one for the minimum values.

Expand the window by adding elements from the right, maintaining the max and min order in the respective deques.

Shrink the window from the left if the current max-min difference exceeds k.

Count all valid subarrays within the current window that have a length of at least min_len.

Time Complexity:

O(n): Each element is added and removed from the deques at most once, resulting in linear time.

Space Complexity:

O(n): The deques can grow up to the length of the array in the worst case.