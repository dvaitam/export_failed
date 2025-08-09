package main

import (
	"bufio"
	"fmt"
	"os"
)

const infinity int64 = 1e18

type DSU struct {
	parent []int
}

func NewDSU(n int) *DSU {
	p := make([]int, n+1)
	for i := 0; i <= n; i++ {
		p[i] = i
	}
	return &DSU{parent: p}
}

func (dsu *DSU) Find(i int) int {
	if dsu.parent[i] == i {
		return i
	}
	dsu.parent[i] = dsu.Find(dsu.parent[i])
	return dsu.parent[i]
}

func (dsu *DSU) Union(i, j int) {
	rootI := dsu.Find(i)
	rootJ := dsu.Find(j)
	if rootI != rootJ {
		dsu.parent[rootI] = rootJ
	}
}

type Edge struct {
	u, v, x int
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var n, m, k int
	fmt.Fscan(in, &n, &m, &k)

	c := make([]int, k)
	for i := 0; i < k; i++ {
		fmt.Fscan(in, &c[i])
	}

	typeOf := make([]int, n+1)
	typeStart := make([]int, k+1)
	currentIdx := 1
	for i := 0; i < k; i++ {
		typeStart[i+1] = currentIdx
		for j := 0; j < c[i]; j++ {
			typeOf[currentIdx+j] = i + 1
		}
		currentIdx += c[i]
	}

	edges := make([]Edge, m)
	for i := 0; i < m; i++ {
		fmt.Fscan(in, &edges[i].u, &edges[i].v, &edges[i].x)
	}

	dsu := NewDSU(n)
	for _, edge := range edges {
		if edge.x == 0 {
			dsu.Union(edge.u, edge.v)
		}
	}

	isCorrect := true
	typeToRoot := make(map[int]int)

	for i := 1; i <= k; i++ {
		startIdx := typeStart[i]
		root := dsu.Find(startIdx)
		typeToRoot[i] = root

		for j := 1; j < c[i-1]; j++ {
			if dsu.Find(startIdx+j) != root {
				isCorrect = false
				break
			}
		}
		if !isCorrect {
			break
		}
	}

	if !isCorrect {
		fmt.Fprintln(out, "No")
		return
	}

	fmt.Fprintln(out, "Yes")
	dist := make([][]int64, k)
	for i := 0; i < k; i++ {
		dist[i] = make([]int64, k)
		for j := 0; j < k; j++ {
			if i == j {
				dist[i][j] = 0
			} else {
				dist[i][j] = infinity
			}
		}
	}

	for i := 1; i <= k; i++ {
		for j := i + 1; j <= k; j++ {
			if typeToRoot[i] == typeToRoot[j] {
				dist[i-1][j-1] = 0
				dist[j-1][i-1] = 0
			}
		}
	}

	for _, edge := range edges {
		rootU := dsu.Find(edge.u)
		rootV := dsu.Find(edge.v)
		if rootU != rootV {
			typeU := typeOf[edge.u] - 1
			typeV := typeOf[edge.v] - 1
			cost := int64(edge.x)

			if dist[typeU][typeV] > cost {
				dist[typeU][typeV] = cost
			}
			if dist[typeV][typeU] > cost {
				dist[typeV][typeU] = cost
			}
		}
	}

	for p := 0; p < k; p++ {
		for i := 0; i < k; i++ {
			for j := 0; j < k; j++ {
				if dist[i][p] != infinity && dist[p][j] != infinity {
					newDist := dist[i][p] + dist[p][j]
					if dist[i][j] > newDist {
						dist[i][j] = newDist
					}
				}
			}
		}
	}

	for i := 0; i < k; i++ {
		for j := 0; j < k; j++ {
			val := dist[i][j]
			if val == infinity {
				val = -1
			}
			fmt.Fprintf(out, "%d ", val)
		}
		fmt.Fprintln(out)
	}
}