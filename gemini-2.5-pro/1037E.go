package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var scanner *bufio.Scanner
var writer *bufio.Writer

func nextInt() int {
	scanner.Scan()
	i, _ := strconv.Atoi(scanner.Text())
	return i
}

func main() {
	scanner = bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	writer = bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	n := nextInt()
	m := nextInt()
	k := nextInt()

	edges := make([][2]int, m)
	adj := make([][]int, n+1)
	degree := make([]int, n+1)

	for i := 0; i < m; i++ {
		u, v := nextInt(), nextInt()
		edges[i] = [2]int{u, v}
		adj[u] = append(adj[u], v)
		adj[v] = append(adj[v], u)
		degree[u]++
		degree[v]++
	}

	active := make([]bool, n+1)
	for i := 1; i <= n; i++ {
		active[i] = true
	}

	currentSize := n
	q := make([]int, 0, n)

	for i := 1; i <= n; i++ {
		if degree[i] < k {
			q = append(q, i)
			active[i] = false
		}
	}

	head := 0
	for head < len(q) {
		u := q[head]
		head++
		currentSize--

		for _, v := range adj[u] {
			if active[v] {
				degree[v]--
				if degree[v] < k {
					q = append(q, v)
					active[v] = false
				}
			}
		}
	}

	ans := make([]int, m)
	if m > 0 {
		ans[m-1] = currentSize
	}

	for i := m - 1; i > 0; i-- {
		u, v := edges[i][0], edges[i][1]

		if active[u] && active[v] {
			degree[u]--
			degree[v]--

			qRemove := make([]int, 0)
			if degree[u] < k {
				qRemove = append(qRemove, u)
				active[u] = false
			}
			if degree[v] < k {
				qRemove = append(qRemove, v)
				active[v] = false
			}

			headRemove := 0
			for headRemove < len(qRemove) {
				x := qRemove[headRemove]
				headRemove++
				currentSize--

				for _, y := range adj[x] {
					if active[y] {
						degree[y]--
						if degree[y] < k {
							qRemove = append(qRemove, y)
							active[y] = false
						}
					}
				}
			}
		}
		ans[i-1] = currentSize
	}

	for i := 0; i < m; i++ {
		fmt.Fprintln(writer, ans[i])
	}
}