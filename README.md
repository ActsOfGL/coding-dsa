***
# Data Structure and Algorithms
***


## This repo shall help the developer to understand DSA by solving and implementing through code



### The repo shall accept any coding language but by default, it's currently using Rust



======================================================
# 📘 DSA Interview Practice in Rust

This project is a curated archive of Data Structures and Algorithms (DSA) problems solved using idiomatic Rust. Each problem is structured like a real MAANG-style interview, gradually increasing in complexity to help build intuition and pattern recognition for technical interviews.

---

## ✅ Solved Problems

### 1. Two Sum
- **Difficulty:** Easy  
- **Pattern:** HashMap  
- **Description:** Given an array of integers and a target, return the indices of the two numbers that add up to the target.  
- **Function:** `two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>`

---

### 2. Group Anagrams
- **Difficulty:** Medium  
- **Pattern:** HashMap + Sorting  
- **Description:** Group all anagrams from a list of strings.  
- **Function:** `group_anagrams(words: Vec<&str>) -> Vec<Vec<String>>`

---

### 3. Contains Duplicates
- **Difficulty:** Easy  
- **Pattern:** HashSet  
- **Description:** Check if any value appears more than once in the array.  
- **Function:** `contains_duplicates(nums: &[i32]) -> bool`

---

### 4. Contains Nearby Duplicate
- **Difficulty:** Medium  
- **Pattern:** HashMap (value to index tracking)  
- **Description:** Return `true` if a duplicate exists within `k` indices of each other.  
- **Function:** `contains_close_duplicates(nums: &[i32], distance: i32) -> bool`

---

### 5. Has Nearby Divisible Duplicate
- **Difficulty:** Medium  
- **Pattern:** HashMap + Divisibility filter  
- **Description:** Return `true` if there are two equal numbers divisible by a given `divisor` and within a `distance` of `k` between indices.  
- **Function:** `has_nearby_divisible_duplicate(nums: &[i32], distance: i32, divisor: i32) -> bool`

---

### 🧩 More Coming Soon...
Next challenges will build from these patterns to introduce sliding windows, prefix sums, two pointers, and more — leading up to hard-level problems.

---

## 🦀 Why Rust?
Rust forces strict ownership and borrowing rules, which helps develop clean logic and tight memory safety — all while building interview-ready problem-solving skills.
