# ✅ Question 2: Subarray Count with Zeros and Negatives Allowed

## 🧠 Description:
Given an array that may contain zeros or negative values, count how many contiguous subarrays have a sum equal to k.

## 💡 Insight:
We can’t use a shrinking window like before, because the sum can shrink or grow unpredictably. Instead, use a prefix sum + hashmap approach. If the difference between the current prefix sum and k has been seen before, the number of such occurrences equals the number of valid subarrays ending at the current index.