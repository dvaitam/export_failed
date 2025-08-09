package main

import (
	"bufio"
	"fmt"
	"os"
)

func isPrime(n int64) bool {
	if n < 2 {
		return false
	}
	small := []int64{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}
	for _, p := range small {
		if n == p {
			return true
		}
		if n%p == 0 {
			return false
		}
	}
	for i := int64(41); i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var n, m int
	fmt.Fscan(in, &n, &m)

	baseSum := int64(n - 2)
	var w int64 = 1
	for {
		if isPrime(baseSum + w) {
			break
		}
		w++
	}
	primeSum := baseSum + w

	out := bufio.NewWriter(os.Stdout)
	fmt.Fprintf(out, "%d %d\n", primeSum, primeSum)

	edgesAdded := 0
	// path edges with weight 1
	for i := 1; i <= n-2; i++ {
		fmt.Fprintf(out, "%d %d 1\n", i, i+1)
		edgesAdded++
	}
	// last path edge with weight w
	fmt.Fprintf(out, "%d %d %d\n", n-1, n, w)
	edgesAdded++

	// extra edges with large weight
	const bigW = 1000000000
	for i := 1; edgesAdded < m && i <= n; i++ {
		for j := i + 2; edgesAdded < m && j <= n; j++ {
			fmt.Fprintf(out, "%d %d %d\n", i, j, bigW)
			edgesAdded++
		}
	}
	out.Flush()
}