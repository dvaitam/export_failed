package main

import (
	"bufio"
	"fmt"
	"os"
)

type Pair struct {
	x, y int
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var n, m int
	if _, err := fmt.Fscan(in, &n, &m); err != nil {
		return
	}

	coords := make([][]Pair, n+1) // 1-based indexing for colors

	// base rook for every color on its own unique row
	for i := 1; i <= n; i++ {
		coords[i] = append(coords[i], Pair{i, i})
	}

	// extra rooks for every harmonizing pair
	for idx := 0; idx < m; idx++ {
		var a, b int
		fmt.Fscan(in, &a, &b)
		row := 1000 + idx // unique row for each pair
		coords[a] = append(coords[a], Pair{a, row})
		coords[b] = append(coords[b], Pair{b, row})
	}

	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	for i := 1; i <= n; i++ {
		fmt.Fprintln(out, len(coords[i]))
		for _, p := range coords[i] {
			fmt.Fprintln(out, p.x, p.y)
		}
	}
}