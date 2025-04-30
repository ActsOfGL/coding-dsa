# ✅ Question 1: Subarray Count with Positive Values Only

## 🧠 Description:
Given an array of only positive integers, count how many contiguous subarrays have a sum equal to k. A subarray can have a length of 1 up to the full length of the array.

## 💡 Insight:
Because the input contains only positive values, we can use a two-pointer sliding window. If the current sum becomes too large, shrink the window from the left. If it's smaller, expand it from the right.