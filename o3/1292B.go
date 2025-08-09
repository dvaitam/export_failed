package main

import (
	"bufio"
	"fmt"
	"os"
)

type Point struct {
	x, y int64
}

func abs(a int64) int64 {
	if a < 0 {
		return -a
	}
	return a
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	in := bufio.NewReader(os.Stdin)

	var x0, y0, ax, ay, bx, by int64
	fmt.Fscan(in, &x0, &y0, &ax, &ay, &bx, &by)

	var xs, ys, t int64
	fmt.Fscan(in, &xs, &ys, &t)

	// Generate useful nodes
	points := make([]Point, 0, 64)
	x, y := x0, y0
	for {
		points = append(points, Point{x, y})

		// Prepare next
		nx := ax*x + bx
		ny := ay*y + by

		// If next is certainly unreachable, stop
		if nx > xs+t && ny > ys+t {
			break
		}
		// Also stop if next might overflow int64 (safe guard)
		if nx > 4e18 || ny > 4e18 {
			break
		}
		x, y = nx, ny
	}

	n := len(points)
	best := 0

	for i := 0; i < n; i++ {
		// distance from start to i
		startToI := abs(points[i].x-xs) + abs(points[i].y-ys)

		for l := 0; l <= i; l++ {
			for r := i; r < n; r++ {
				distLR := (points[r].x - points[l].x) + (points[r].y - points[l].y)
				distIL := (points[i].x - points[l].x) + (points[i].y - points[l].y)
				distIR := (points[r].x - points[i].x) + (points[r].y - points[i].y)

				total := startToI + distLR + min(distIL, distIR)
				if total <= t {
					best = max(best, r-l+1)
				}
			}
		}
	}

	fmt.Println(best)
}