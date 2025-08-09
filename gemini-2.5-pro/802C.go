package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const inf int64 = 1e18

type Edge struct {
	to, cap int
	cost    int64
	rev     int
}

var graph [][]Edge
var dist []int64
var prevv, preve []int
var num_nodes int

func addEdge(from, to, cap int, cost int64) {
	graph[from] = append(graph[from], Edge{to, cap, cost, len(graph[to])})
	graph[to] = append(graph[to], Edge{from, 0, -cost, len(graph[from]) - 1})
}

func minCostFlow(s, t, f int) int64 {
	var res int64 = 0
	dist = make([]int64, num_nodes)
	prevv = make([]int, num_nodes)
	preve = make([]int, num_nodes)

	for f > 0 {
		for i := range dist {
			dist[i] = inf
		}
		dist[s] = 0

		inQueue := make([]bool, num_nodes)
		queue := make([]int, 0, num_nodes)
		queue = append(queue, s)
		inQueue[s] = true

		for len(queue) > 0 {
			u := queue[0]
			queue = queue[1:]
			inQueue[u] = false

			for i, e := range graph[u] {
				if e.cap > 0 && dist[e.to] > dist[u]+e.cost {
					dist[e.to] = dist[u] + e.cost
					prevv[e.to] = u
					preve[e.to] = i
					if !inQueue[e.to] {
						queue = append(queue, e.to)
						inQueue[e.to] = true
					}
				}
			}
		}

		if dist[t] == inf {
			break
		}

		d := f
		for v := t; v != s; v = prevv[v] {
			if d > graph[prevv[v]][preve[v]].cap {
				d = graph[prevv[v]][preve[v]].cap
			}
		}

		f -= d
		res += int64(d) * dist[t]

		for v := t; v != s; v = prevv[v] {
			e := &graph[prevv[v]][preve[v]]
			e.cap -= d
			rev_e := &graph[v][e.rev]
			rev_e.cap += d
		}
	}
	return res
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	writer := bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	line, _ := reader.ReadString('\n')
	parts := strings.Fields(line)
	n, _ := strconv.Atoi(parts[0])
	k, _ := strconv.Atoi(parts[1])

	line, _ = reader.ReadString('\n')
	parts = strings.Fields(line)
	a := make([]int, n)
	for i := 0; i < n; i++ {
		a[i], _ = strconv.Atoi(parts[i])
	}

	line, _ = reader.ReadString('\n')
	parts = strings.Fields(line)
	c_input := make([]int64, n)
	for i := 0; i < n; i++ {
		c_input[i], _ = strconv.ParseInt(parts[i], 10, 64)
	}

	costs := make([]int64, n+1)
	for i := 0; i < n; i++ {
		costs[i+1] = c_input[i]
	}

	source := n
	sink := n + 1
	num_nodes = n + 2
	graph = make([][]Edge, num_nodes)

	addEdge(source, 0, k, 0)
	for i := 0; i < n; i++ {
		addEdge(i, i+1, k, 0)
	}
	addEdge(n, sink, k, 0)

	var totalPossibleCost int64 = 0
	lastRequest := make(map[int]int)

	for i := 0; i < n; i++ {
		bookID := a[i]
		bookCost := costs[bookID]

		totalPossibleCost += bookCost

		if prevIdx, ok := lastRequest[bookID]; ok {
			addEdge(prevIdx, i, 1, -bookCost)
		}
		lastRequest[bookID] = i
	}

	maxSavings := minCostFlow(source, sink, k)

	fmt.Fprintln(writer, totalPossibleCost+maxSavings)
}