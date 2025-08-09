package main

import (
	"fmt"
)

func abs(n int64) int64 {
	if n < 0 {
		return -n
	}
	return n
}

type Point struct {
	x, y int64
}

func dist(p1, p2 Point) int64 {
	return abs(p1.x-p2.x) + abs(p1.y-p2.y)
}

func main() {
	var x0, y0, ax, ay, bx, by int64
	fmt.Scan(&x0, &y0, &ax, &ay, &bx, &by)

	var xs, ys, t int64
	fmt.Scan(&xs, &ys, &t)

	startPoint := Point{xs, ys}

	nodes := []Point{}
	cx, cy := x0, y0

	limit := int64(40000000000000000)

	for {
		if cx > limit || cy > limit {
			break
		}

		nodes = append(nodes, Point{cx, cy})

		nx := ax*cx + bx
		ny := ay*cy + by

		if nx < cx || ny < cy {
			break
		}
		cx, cy = nx, ny
	}

	maxNodes := 0
	n := len(nodes)

	for i := 0; i < n; i++ {
		for j := i; j < n; j++ {

			pi := nodes[i]
			pj := nodes[j]
			dist_ij := dist(pi, pj)
			numCollected := j - i + 1

			for k := i; k <= j; k++ {
				pk := nodes[k]

				time1 := dist(startPoint, pk) + dist(pk, pi) + dist_ij
				if time1 <= t {
					if numCollected > maxNodes {
						maxNodes = numCollected
					}
				}

				time2 := dist(startPoint, pk) + dist(pk, pj) + dist_ij
				if time2 <= t {
					if numCollected > maxNodes {
						maxNodes = numCollected
					}
				}
			}
		}
	}

	fmt.Println(maxNodes)
}