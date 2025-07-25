# Project 030 – Generic Binary Search CLI


## What I Built
An interactive Rust CLI that searches for a user-supplied value in a predefined sorted list of strings or numbers using a generic binary search function that works on any sorted list of items that implement PartialOrd.

## What I Learned


## Notes
Binary search is an efficient algorithm used to find the position of a target value within a sorted array or list. It works by repeatedly dividing the search interval in half.

The core formula in a binary search algorithm is used to calculate the middle index of the current search space. This middle index is then used to compare the element at that position with the target value.
The formula for calculating the middle index is:

```
mid = low + (high - low) / 2
```







