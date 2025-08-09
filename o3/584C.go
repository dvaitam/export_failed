package main

import (
	"bufio"
	"fmt"
	"os"
)

func diffChar(a, b byte) byte {
	for c := byte('a'); c <= 'z'; c++ {
		if c != a && (b == 0 || c != b) {
			return c
		}
	}
	return 'a'
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var n, t int
	if _, err := fmt.Fscan(in, &n, &t); err != nil {
		return
	}
	var s1, s2 string
	fmt.Fscan(in, &s1)
	fmt.Fscan(in, &s2)

	b1 := []byte(s1)
	b2 := []byte(s2)

	diffIdx := make([]int, 0)
	sameIdx := make([]int, 0)

	for i := 0; i < n; i++ {
		if b1[i] != b2[i] {
			diffIdx = append(diffIdx, i)
		} else {
			sameIdx = append(sameIdx, i)
		}
	}

	d := len(diffIdx)
	s := n - d

	low := max(0, d-t)
	high := min(d/2, t-d+s)

	if low > high {
		fmt.Println(-1)
		return
	}

	x := low
	_ = d - 2*x
	z := t - d + x

	res := make([]byte, n)

	// First x differing positions equal s1
	for i := 0; i < x; i++ {
		idx := diffIdx[i]
		res[idx] = b1[idx]
	}
	// Next x differing positions equal s2
	for i := x; i < 2*x; i++ {
		idx := diffIdx[i]
		res[idx] = b2[idx]
	}
	// Remaining differing positions differ from both
	for i := 2 * x; i < d; i++ {
		idx := diffIdx[i]
		res[idx] = diffChar(b1[idx], b2[idx])
	}
	// First z same positions differ from both
	for i := 0; i < z; i++ {
		idx := sameIdx[i]
		res[idx] = diffChar(b1[idx], 0)
	}
	// Remaining same positions equal original character
	for i := z; i < s; i++ {
		idx := sameIdx[i]
		res[idx] = b1[idx]
	}

	fmt.Println(string(res))
}