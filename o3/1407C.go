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

	var n int
	fmt.Fscan(in, &n)

	ans := make([]int, n+1) // 1-based
	cand := 1               // current candidate for maximum value

	for i := 2; i <= n; i++ {
		// query cand % i
		fmt.Fprintf(out, "? %d %d\n", cand, i)
		out.Flush()
		var x int
		fmt.Fscan(in, &x)

		// query i % cand
		fmt.Fprintf(out, "? %d %d\n", i, cand)
		out.Flush()
		var y int
		fmt.Fscan(in, &y)

		if x > y {
			// cand is smaller; we now know its value
			ans[cand] = x
			cand = i
		} else {
			// i is smaller; we now know its value
			ans[i] = y
		}
	}

	// the remaining position holds the maximum value, which is n
	ans[cand] = n

	// output the permutation
	fmt.Fprint(out, "!")
	for i := 1; i <= n; i++ {
		fmt.Fprintf(out, " %d", ans[i])
	}
	fmt.Fprintln(out)
}