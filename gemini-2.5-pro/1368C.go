package main

import (
	"bufio"
	"fmt"
	"os"
)

type point struct {
	x, y int
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	writer := bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	var n int
	fmt.Fscan(reader, &n)

	cells := make(map[point]struct{})

	// The construction consists of a chain of n+1 2x2 blocks.
	// This creates n "junction" cells where blocks overlap. Each junction
	// cell will have 4 neighbors. All other cells will have 2 neighbors.
	// The i-th block (0-indexed) is a 2x2 square with its bottom-left
	// corner at coordinates (i, i).
	for i := 0; i <= n; i++ {
		// The four cells of the 2x2 block starting at (i, i)
		cells[point{i, i}] = struct{}{}
		cells[point{i + 1, i}] = struct{}{}
		cells[point{i, i + 1}] = struct{}{}
		cells[point{i + 1, i + 1}] = struct{}{}
	}

	// The total number of unique cells is the size of the map.
	// This will be 3*n + 4.
	fmt.Fprintln(writer, len(cells))

	// Print the coordinates of each unique cell.
	for p := range cells {
		fmt.Fprintln(writer, p.x, p.y)
	}
}