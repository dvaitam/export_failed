package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	var n int
	fmt.Sscan(scanner.Text(), &n)
	if n == 0 {
		return
	}
	var groups [][]int
	g := make([]int, n)
	for i := 0; i < n; i++ {
		g[i] = i + 1
	}
	groups = append(groups, g)
	var chosen []int
	for len(groups) < n {
		bestY := -1
		maxInc := -1
		for y := 1; y <= n; y++ {
			inc := 0
			for _, gg := range groups {
				d := 0
				for _, x := range gg {
					if x%y == 0 {
						d++
					}
				}
				if d > 0 && d < len(gg) {
					inc++
				}
			}
			if inc > maxInc {
				maxInc = inc
				bestY = y
			}
		}
		if maxInc <= 0 {
			break
		}
		chosen = append(chosen, bestY)
		var newG [][]int
		for _, gg := range groups {
			var yes, no []int
			for _, x := range gg {
				if x%bestY == 0 {
					yes = append(yes, x)
				} else {
					no = append(no, x)
				}
			}
			if len(yes) > 0 {
				newG = append(newG, yes)
			}
			if len(no) > 0 {
				newG = append(newG, no)
			}
		}
		groups = newG
	}
	fmt.Print(len(chosen))
	for _, y := range chosen {
		fmt.Print(" ", y)
	}
	fmt.Println()
}