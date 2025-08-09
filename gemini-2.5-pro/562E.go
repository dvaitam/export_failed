package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

type Pair struct {
	a, b int64
}

var scanner *bufio.Scanner

func initScanner() {
	scanner = bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	const bufSize = 1024 * 1024
	buf := make([]byte, bufSize)
	scanner.Buffer(buf, bufSize)
}

func nextInt() int {
	scanner.Scan()
	i, _ := strconv.Atoi(scanner.Text())
	return i
}

func nextInt64() int64 {
	scanner.Scan()
	i, _ := strconv.ParseInt(scanner.Text(), 10, 64)
	return i
}

func main() {
	initScanner()

	n := nextInt()
	m := nextInt()
	_, _ = nextInt64(), nextInt64() // Initial x and y are not needed

	maxMoves := make([]Pair, n)
	for i := 0; i < n; i++ {
		maxMoves[i] = Pair{a: nextInt64(), b: nextInt64()}
	}

	minMoves := make([]Pair, m)
	for i := 0; i < m; i++ {
		minMoves[i] = Pair{a: nextInt64(), b: nextInt64()}
	}

	// Build the Pareto frontier of Min's moves.
	// A move (c, d) is on the frontier if no other move (c', d') has c' >= c and d' >= d.
	sort.Slice(minMoves, func(i, j int) bool {
		if minMoves[i].a != minMoves[j].a {
			return minMoves[i].a < minMoves[j].a
		}
		return minMoves[i].b < minMoves[j].b
	})

	frontier := make([]Pair, 0)
	maxB := int64(-1)
	for i := m - 1; i >= 0; i-- {
		if minMoves[i].b > maxB {
			frontier = append(frontier, minMoves[i])
			maxB = minMoves[i].b
		}
	}
	// The frontier is now sorted by 'a' descending, and 'b' is strictly ascending.

	for _, move := range maxMoves {
		a, b := move.a, move.b

		// We need to check if there is a move (c,d) in the frontier such that c > a and d > b.
		// `sort.Search` finds the first index `idx` in the frontier where frontier[idx].a <= a.
		idx := sort.Search(len(frontier), func(i int) bool {
			return frontier[i].a <= a
		})

		if idx == 0 {
			// No c > a exists in the frontier. This is because frontier[0].a is the largest 'a',
			// and if it's <= a, no other element's 'a' can be > a. This move is safe.
			fmt.Println("Max")
			return
		}

		// The points in the frontier with `c > a` are at indices 0 to idx-1.
		// Since the 'b' values are ascending in the frontier (b_0 < b_1 < ...), to check
		// if any of these points have `d > b`, we only need to check the one with the largest 'b',
		// which is at index `idx-1`.
		// If frontier[idx-1].b <= b, then all other points from 0 to idx-2 will also have their 'b'
		// value <= b, making the move safe.
		if frontier[idx-1].b <= b {
			fmt.Println("Max")
			return
		}
	}

	// If no safe move was found for Max after checking all possibilities, Min wins.
	fmt.Println("Min")
}