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
		var k int64
		fmt.Fscan(in, &n, &k)

		a := make([]int64, n)
		var sum int64
		for i := 0; i < n; i++ {
			fmt.Fscan(in, &a[i])
			sum += a[i]
		}

		lo := int64(n)
		hi := sum + k

		for lo < hi {
			mid := (lo + hi + 1) / 2
			q := mid / int64(n)
			r := mid % int64(n)

			var need, extra int64
			for _, v := range a {
				if v < q {
					need += q - v
					if need > k {
						break
					}
				} else {
					extra += v - q
				}
			}

			if need > k {
				hi = mid - 1
				continue
			}

			cost := need
			if extra < r {
				cost += r - extra
			}

			if cost <= k {
				lo = mid
			} else {
				hi = mid - 1
			}
		}

		ans := lo - int64(n) + 1
		fmt.Fprintln(out, ans)
	}
}