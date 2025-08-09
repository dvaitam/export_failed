package main

import (
	"bufio"
	"fmt"
	"os"
)

var reader = bufio.NewReader(os.Stdin)
var writer = bufio.NewWriter(os.Stdout)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func solve() {
	var n, k int
	fmt.Fscan(reader, &n, &k)
	var s string
	fmt.Fscan(reader, &s)

	totalOnes := 0
	onesAtRemainder := make([]int, k)
	for i := 0; i < n; i++ {
		if s[i] == '1' {
			totalOnes++
			onesAtRemainder[i%k]++
		}
	}

	minTotalFlips := totalOnes

	for r := 0; r < k; r++ {
		onesInSub := onesAtRemainder[r]
		if onesInSub == 0 {
			continue
		}

		// Cost to turn '1's in other subsequences to '0's.
		costA := totalOnes - onesInSub

		// costB is the minimum flips needed within the r-subsequence.
		// Option 1: Turn all lamps in this subsequence to '0'.
		minFlipsInSub := onesInSub

		// Option 2: Have a contiguous block of '1's.
		// We use a dynamic programming approach with an optimization.
		// The cost to make a prefix of the subsequence periodic ending in '1'
		// can be calculated efficiently by tracking a value `minG`.
		minG := 0
		currentOnesPrefix := 0
		posCount := 0

		for i := r; i < n; i += k {
			// Let `f(j) = 2 * ones_prefix[j-1] - j`.
			// `minG` tracks `min_{0 <= j <= posCount} f(j)`.
			// `currentOnesPrefix` at this point is the count up to `posCount-1`.
			gVal := 2*currentOnesPrefix - posCount
			minG = min(minG, gVal)

			// Update prefix sum for the current position `posCount`.
			if s[i] == '1' {
				currentOnesPrefix++
			}

			// Cost for the prefix `0..posCount` to be periodic, ending in a block of '1's.
			costForBlockAtI := (posCount + 1 - currentOnesPrefix) + minG

			// Total cost for the entire subsequence if the block of '1's ends at `posCount`.
			// This includes the cost to turn the remaining part of the subsequence to '0's.
			finalCost := costForBlockAtI + (onesInSub - currentOnesPrefix)

			minFlipsInSub = min(minFlipsInSub, finalCost)

			posCount++
		}

		costB := minFlipsInSub

		currentTotalCost := costA + costB
		minTotalFlips = min(minTotalFlips, currentTotalCost)
	}

	fmt.Fprintln(writer, minTotalFlips)
}

func main() {
	defer writer.Flush()
	var t int
	fmt.Fscan(reader, &t)
	for i := 0; i < t; i++ {
		solve()
	}
}