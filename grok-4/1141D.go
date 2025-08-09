package main

import (
	"bufio"
	"fmt"
	"os"
)

const INF = 300010

var level, iter []int

type edge struct {
	to, cap, rev int
}

var g [][]edge

func addEdge(from, to, cap int) {
	g[from] = append(g[from], edge{to, cap, len(g[to])})
	g[to] = append(g[to], edge{from, 0, len(g[from]) - 1})
}

func bfs(s int) {
	level = make([]int, len(g))
	for i := range level {
		level[i] = -1
	}
	level[s] = 0
	q := []int{s}
	for len(q) > 0 {
		v := q[0]
		q = q[1:]
		for _, e := range g[v] {
			if e.cap > 0 && level[e.to] < 0 {
				level[e.to] = level[v] + 1
				q = append(q, e.to)
			}
		}
	}
}

func dfs(v, t, f int) int {
	if v == t {
		return f
	}
	for i := iter[v]; i < len(g[v]); i++ {
		iter[v] = i
		e := &g[v][i]
		if e.cap > 0 && level[v] < level[e.to] {
			d := dfs(e.to, t, min(f, e.cap))
			if d > 0 {
				e.cap -= d
				g[e.to][e.rev].cap += d
				return d
			}
		}
	}
	return 0
}

func maxflow(s, t int) int {
	flow := 0
	for {
		bfs(s)
		if level[t] < 0 {
			return flow
		}
		iter = make([]int, len(g))
		for {
			f := dfs(s, t, INF)
			if f == 0 {
				break
			}
			flow += f
		}
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var n int
	fmt.Fscan(in, &n)
	var l, r string
	fmt.Fscan(in, &l)
	fmt.Fscan(in, &r)

	var freqL, freqR [27]int
	var leftList, rightList [27][]int
	for i := 0; i < n; i++ {
		cl := l[i]
		var idx int
		if cl == '?' {
			idx = 0
		} else {
			idx = 1 + int(cl-'a')
		}
		freqL[idx]++
		leftList[idx] = append(leftList[idx], i+1)

		cr := r[i]
		if cr == '?' {
			idx = 0
		} else {
			idx = 1 + int(cr-'a')
		}
		freqR[idx]++
		rightList[idx] = append(rightList[idx], i+1)
	}

	const V = 56
	g = make([][]edge, V)

	// Source 0, Sink 1, Left 2-28, Right 29-55
	for idx := 0; idx < 27; idx++ {
		nodeL := 2 + idx
		addEdge(0, nodeL, freqL[idx])
		nodeR := 29 + idx
		addEdge(nodeR, 1, freqR[idx])
	}

	for idxL := 0; idxL < 27; idxL++ {
		nodeL := 2 + idxL
		var cl byte
		if idxL == 0 {
			cl = '?'
		} else {
			cl = 'a' + byte(idxL-1)
		}
		for idxR := 0; idxR < 27; idxR++ {
			nodeR := 29 + idxR
			var cr byte
			if idxR == 0 {
				cr = '?'
			} else {
				cr = 'a' + byte(idxR-1)
			}
			if cl == cr || cl == '?' || cr == '?' {
				addEdge(nodeL, nodeR, INF)
			}
		}
	}

	maxflow(0, 1)

	var pairs [][2]int
	var leftPtr, rightPtr [27]int
	for idxL := 0; idxL < 27; idxL++ {
		nodeL := 2 + idxL
		for _, e := range g[nodeL] {
			if e.to >= 29 && e.to <= 55 {
				idxR := e.to - 29
				num := g[e.to][e.rev].cap
				for j := 0; j < num; j++ {
					a := leftList[idxL][leftPtr[idxL]]
					b := rightList[idxR][rightPtr[idxR]]
					pairs = append(pairs, [2]int{a, b})
					leftPtr[idxL]++
					rightPtr[idxR]++
				}
			}
		}
	}

	k := len(pairs)
	fmt.Println(k)
	for _, p := range pairs {
		fmt.Println(p[0], p[1])
	}
}