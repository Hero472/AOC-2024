# Advent of Code 2024 - Day 1

This is my solution for the **Advent of Code 2024**, Day 1 challenge.

## 📝 Description

This program reads input data from a file, processes it, and calculates the total similarity score between two sets of numbers.

### Steps:
1. Read the input file located in `../../day-1/input.txt`.
2. Parse two numbers from each line of the file, separated by 3 spaces.
3. For each number in the left set, count how many times it appears in the right set.
4. Multiply each number by its count and sum the results to calculate the similarity score.