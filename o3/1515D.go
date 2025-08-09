package main

import (
	"bufio"
	"fmt"
	"os"
)

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var t int
	fmt.Fscan(in, &t)
	for ; t > 0; t-- {
		var n, l, r int
		fmt.Fscan(in, &n, &l, &r)

		left := make([]int, n+1)
		right := make([]int, n+1)

		for i := 0; i < l; i++ {
			var c int
			fmt.Fscan(in, &c)
			left[c]++
		}
		for i := 0; i < r; i++ {
			var c int
			fmt.Fscan(in, &c)
			right[c]++
		}

		L, R := l, r
		for i := 1; i <= n; i++ {
			m := min(left[i], right[i])
			left[i] -= m
			right[i] -= m
			L -= m
			R -= m
		}

		var need, pairs, use, ans int
		if L < R {
			need = (R - L) / 2
			for i := 1; i <= n; i++ {
				pairs += right[i] / 2
			}
		} else {
			need = (L - R) / 2
			for i := 1; i <= n; i++ {
				pairs += left[i] / 2
			}
		}

		use = min(pairs, need)
		ans = need + (L+R)/2 - use
		fmt.Println(ans)
	}
}