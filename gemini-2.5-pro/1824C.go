package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

var (
	n       int
	a       []int
	adj     [][]int
	scanner *bufio.Scanner
)

func nextInt() int {
	scanner.Scan()
	i, _ := strconv.Atoi(scanner.Text())
	return i
}

func dfs(u, p int) (map[int]int, int) {
	var children []int
	for _, v := range adj[u] {
		if v != p {
			children = append(children, v)
		}
	}

	if len(children) == 0 {
		deltaMap := make(map[int]int)
		deltaMap[a[u-1]] = -1
		return deltaMap, 1
	}

	type childResult struct {
		deltaMap map[int]int
		baseCost int
	}
	childResults := make([]childResult, len(children))

	for i, v := range children {
		dm, bc := dfs(v, u)
		childResults[i] = childResult{dm, bc}
	}

	sort.Slice(childResults, func(i, j int) bool {
		return len(childResults[i].deltaMap) > len(childResults[j].deltaMap)
	})

	mergedDeltaMap := childResults[0].deltaMap
	mergedBaseCost := childResults[0].baseCost

	for i := 1; i < len(childResults); i++ {
		mergedBaseCost += childResults[i].baseCost
		for x, delta := range childResults[i].deltaMap {
			mergedDeltaMap[x] += delta
		}
	}

	minDelta := 0
	for _, d := range mergedDeltaMap {
		if d < minDelta {
			minDelta = d
		}
	}

	minCostChildren := mergedBaseCost + minDelta
	var xMin int

	if minDelta == 0 {
		xMin = 0
		for {
			d, exists := mergedDeltaMap[xMin]
			if !exists || d == 0 {
				break
			}
			xMin++
		}
	} else {
		for x, d := range mergedDeltaMap {
			if d == minDelta {
				xMin = x
				break
			}
		}
	}

	newBaseCost := minCostChildren + 1
	newDeltaMap := make(map[int]int)
	newDeltaMap[a[u-1]^xMin] = -1

	return newDeltaMap, newBaseCost
}

func main() {
	scanner = bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	scanner.Buffer(make([]byte, 1024*1024), 1024*1024)

	n = nextInt()

	a = make([]int, n)
	for i := 0; i < n; i++ {
		a[i] = nextInt()
	}

	adj = make([][]int, n+1)
	for i := 0; i < n-1; i++ {
		u, v := nextInt(), nextInt()
		adj[u] = append(adj[u], v)
		adj[v] = append(adj[v], u)
	}

	deltaMap, baseCost := dfs(1, 0)

	finalCost := baseCost
	if d, ok := deltaMap[0]; ok {
		finalCost += d
	}

	fmt.Println(finalCost)
}