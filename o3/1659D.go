package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var t int
	fmt.Fscan(in, &t)
	for ; t > 0; t-- {
		var n int
		fmt.Fscan(in, &n)
		c := make([]int, n+1) // 1-based
		for i := 1; i <= n; i++ {
			fmt.Fscan(in, &c[i])
		}

		// special case : every element is 0
		if c[n] == 0 {
			for i := 0; i < n; i++ {
				if i > 0 {
					fmt.Fprint(out, " ")
				}
				fmt.Fprint(out, 0)
			}
			fmt.Fprintln(out)
			continue
		}

		lastMustOne := (c[n] == n) // true  -> a_n must be 1 ; false -> a_n must be 0
		a := make([]int, n+1)      // 1-based
		needZero := make([]bool, n+2) // positions that are forced to be 0 in the future
		prevL := 0                   // L_{j-1}

		for j := 1; j < n; j++ {

			// if this position is already forced to be zero
			if needZero[j] {
				a[j] = 0
				L0 := c[j] + j
				if prevL < L0 {
					prevL = L0
				}
				continue
			}

			// try to put 1 here
			canOne := true
			L1 := c[j] + 1
			if c[j] < j-1 { // F_j would be negative
				canOne = false
			}
			if L1 <= j { // first following zero cannot be here or before
				canOne = false
			}
			if prevL != n+1 {
				if L1 <= prevL {
					canOne = false
				}
			} else {
				if L1 < prevL {
					canOne = false
				}
			}
			if L1 == n && lastMustOne { // position n is forced to be 1
				canOne = false
			}
			if L1 > n+1 {
				canOne = false
			}

			if canOne {
				a[j] = 1
				if L1 <= n {
					needZero[L1] = true
				}
				if prevL < L1 {
					prevL = L1
				}
				continue
			}

			// otherwise put 0
			a[j] = 0
			L0 := c[j] + j
			if prevL < L0 {
				prevL = L0
			}
			// no need to set new requirement, because the zero is already here
		}

		// handle position n
		if needZero[n] && lastMustOne {
			// shouldn't happen because input guarantees existence of answer
		}
		if lastMustOne {
			a[n] = 1
		} else {
			a[n] = 0
		}

		// output
		for i := 1; i <= n; i++ {
			if i > 1 {
				fmt.Fprint(out, " ")
			}
			fmt.Fprint(out, a[i])
		}
		fmt.Fprintln(out)
	}
}