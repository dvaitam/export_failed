package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	var t int
	fmt.Fscan(in, &t)
	for tt := 0; tt < t; tt++ {
		var n int
		var p int64
		var k int
		fmt.Fscan(in, &n, &p, &k)
		a := make([]int, n)
		for i := 0; i < n; i++ {
			fmt.Fscan(in, &a[i])
		}
		sort.Ints(a)
		low := 0
		high := n
		for low < high {
			mid := (low + high + 1) / 2
			if can(mid, p, k, a) {
				low = mid
			} else {
				high = mid - 1
			}
		}
		fmt.Println(low)
	}
}

func can(m int, p int64, k int, a []int) bool {
	if m == 0 {
		return true
	}
	var cost int64 = 0
	var pool int = 0
	for i := m - 1; i >= 0; i-- {
		if pool > 0 {
			pool--
		} else {
			cost += int64(a[i])
			pool += k - 1
			if cost > p {
				return false
			}
		}
	}
	return cost <= p
}