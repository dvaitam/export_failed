package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	var n, k int
	if _, err := fmt.Fscan(in, &n, &k); err != nil {
		return
	}
	L := 0
	for i := 0; i < k; i++ {
		var m int
		fmt.Fscan(in, &m)
		prefix := 0
		for j := 0; j < m; j++ {
			var x int
			fmt.Fscan(in, &x)
			if L == 0 { // only the chain containing 1 matters
				if j == 0 {
					if x == 1 {
						prefix = 1
					}
				} else if prefix == j && x == j+1 {
					prefix++
				}
			}
		}
		if L == 0 && prefix > 0 {
			L = prefix
		}
	}
	var ans int
	if L == 0 {
		ans = 2*n - k - 1
	} else {
		ans = 2*n - k - 2*L + 1
	}
	out := bufio.NewWriter(os.Stdout)
	fmt.Fprintln(out, ans)
	out.Flush()
}