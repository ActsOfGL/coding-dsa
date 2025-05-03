# 🧠 Interview-Style Challenge
You’re working on a backend analytics tool that tracks user activity over time.
You’re given a list of integers where each number represents net user activity change (positive or negative) per minute.
Your task is to find the number of time intervals (i.e., contiguous subarrays) that:

Have a total activity change of exactly target.
Are at least 3 minutes long.

Example input:
```
let activity_changes = [1, 2, -1, 3, -2, 2];
let target = 3;
```
Expected behavior: count how many valid subarrays meet both conditions.