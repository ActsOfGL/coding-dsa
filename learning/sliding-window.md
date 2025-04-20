# ⛓️ Sliding Window vs Prefix Sums

| Technique        | Memory     | Speed  | Good for                     |
|----------------  |------------|--------|------------------------------|
| Sliding Window   | O(1) extra | O(n)   | Real-time updates, streams   |
| Prefix Sum (map) | O(n) extra | O(n)   | Many queries over same array |

---

## 🧪 1. Your Prefix Sum + Subtraction Approach
💡 Concept:
You wanted to precompute prefix sums, then subtract previous sums to get the sum of a window of size k.

arr[i] = sum of elements from i-k+1 to i

✅ Pros:
Once prefix sums are computed, any subarray sum becomes an O(1) lookup!

Useful when:
 - You're asked many subarray sum queries
 - You’re not sliding, just answering range questions fast

❌ Cons:
 - Requires an extra array (prefix sum array)
 - More memory usage
 - A bit overkill if you're only solving for one max sum of size k — in that case, you're doing extra work
---

## 🪟 2. Sliding Window Approach

💡 Concept:
You manually sum the first window, then:

Slide 1 step right:
new_sum = old_sum - nums[i - k] + nums[i]

✅ Pros:
 - No extra memory (just 2 variables)
 - Fast: O(n)
 - Best choice for single-pass problems like: "Find the max sum of any subarray of size k"

❌ Cons:
 - Doesn’t scale to multiple different k queries unless recomputed
 - Only works if window size is fixed and known

 ---

## 🥊 When to Use What

| Problem Type	                                    | Recommended Approach                 |
|---------------------------------------------------|--------------------------------------|
| "What's the max sum of any subarray of size k?"	  | Sliding Window                       |
| "What’s the sum from index i to j, many times?"	  | Prefix Sum Array                     |
| "All subarray sums of size k, multiple queries"	  | Prefix Sum                           |
| "Streaming numbers, maintain a rolling window"	  | Sliding Window (with Queue or Deque) |

## 📌 TL;DR
Prefix Sum is like building a calculator for fast future lookups.

Sliding Window is like reusing the last answer to get the next one faster — no extra memory.

