package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Solver struct {
	n, m     int
	adj, revAdj []map[int]bool
}

func NewSolver(n, m int) *Solver {
	adj := make([]map[int]bool, n+1)
	revAdj := make([]map[int]bool, n+1)
	for i := 1; i <= n; i++ {
		adj[i] = make(map[int]bool)
		revAdj[i] = make(map[int]bool)
	}
	return &Solver{n: n, m: m, adj: adj, revAdj: revAdj}
}

func (s *Solver) AddEdge(u, v int) {
	if !s.adj[u][v] {
		s.adj[u][v] = true
		s.revAdj[v][u] = true
	}
}

func (s *Solver) Solve() {
	// Infinite check
	canReachOne := make([]bool, s.n+1)
	q := []int{1}
	canReachOne[1] = true
	head := 0
	for head < len(q) {
		u := q[head]
		head++
		for v := range s.revAdj[u] {
			if !canReachOne[v] {
				canReachOne[v] = true
				q = append(q, v)
			}
		}
	}

	for i := 2; i <= s.n; i++ {
		if !canReachOne[i] {
			fmt.Println("INFINITE")
			return
		}
	}

	fmt.Println("FINITE")

	// Tarjan's algorithm for SCC
	ids := make([]int, s.n+1)
	low := make([]int, s.n+1)
	onStack := make([]bool, s.n+1)
	stack := []int{}
	timer := 0
	
	var tarjan func(at int)
	tarjan = func(at int) {
		stack = append(stack, at)
		onStack[at] = true
		timer++
		ids[at], low[at] = timer, timer

		for to := range s.adj[at] {
			if ids[to] == 0 {
				tarjan(to)
			}
			if onStack[to] {
				if low[to] < low[at] {
					low[at] = low[to]
				}
			}
		}

		if ids[at] == low[at] {
			for {
				node := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				onStack[node] = false
				low[node] = ids[at]
				if node == at {
					break
				}
			}
		}
	}

	for i := 1; i <= s.n; i++ {
		if ids[i] == 0 {
			tarjan(i)
		}
	}

	sccs := make(map[int][]int)
	sccIDMap := make(map[int]int)
	sccIDCounter := 1
	for i := 1; i <= s.n; i++ {
		sccRoot := low[i]
		if _, ok := sccIDMap[sccRoot]; !ok {
			sccIDMap[sccRoot] = sccIDCounter
			sccIDCounter++
		}
		sccID := sccIDMap[sccRoot]
		sccs[sccID] = append(sccs[sccID], i)
	}

	sccOf := make([]int, s.n+1)
	for id, nodes := range sccs {
		sort.Ints(nodes)
		sccs[id] = nodes
		for _, node := range nodes {
			sccOf[node] = id
		}
	}
	
	sccAdj := make([]map[int]bool, sccIDCounter)
	for i := 1; i < sccIDCounter; i++ {
		sccAdj[i] = make(map[int]bool)
	}
	for u := 1; u <= s.n; u++ {
		uSCC := sccOf[u]
		for v := range s.adj[u] {
			vSCC := sccOf[v]
			if uSCC != vSCC {
				if !sccAdj[uSCC][vSCC] {
					sccAdj[uSCC][vSCC] = true
				}
			}
		}
	}

	// Rank calculation
	scc1 := sccOf[1]
	ranks := make([]int, sccIDCounter)
	for i := range ranks {
		ranks[i] = -1
	}
	
	var computeRank func(sccID int) int
	computeRank = func(sccID int) int {
		if ranks[sccID] != -1 {
			return ranks[sccID]
		}
		if sccID == scc1 {
			ranks[sccID] = 0
			return 0
		}
		maxChildRank := -1
		for childSCC := range sccAdj[sccID] {
			childRank := computeRank(childSCC)
			if childRank > maxChildRank {
				maxChildRank = childRank
			}
		}
		ranks[sccID] = 1 + maxChildRank
		return ranks[sccID]
	}
	
	maxRank := 0
	for i := 1; i < sccIDCounter; i++ {
		r := computeRank(i)
		if r > maxRank {
			maxRank = r
		}
	}

	layers := make([][]int, maxRank+1)
	for i := 1; i < sccIDCounter; i++ {
		if ranks[i] >= 0 {
			layers[ranks[i]] = append(layers[ranks[i]], i)
		}
	}

	for d := 0; d <= maxRank; d++ {
		sort.Slice(layers[d], func(i, j int) bool {
			return sccs[layers[d][i]][0] < sccs[layers[d][j]][0]
		})
	}

	// Construct sequence
	seq := []int{1}
	for d := 1; d <= maxRank; d++ {
		layerBlock := []int{}
		for _, sccID := range layers[d] {
			sccNodes := sccs[sccID]
			k := len(sccNodes)
			for i := 0; i < k; i++ {
				layerBlock = append(layerBlock, sccNodes...)
			}
		}
		newSeq := make([]int, 0, len(layerBlock)*2+len(seq))
		newSeq = append(newSeq, layerBlock...)
		newSeq = append(newSeq, seq...)
		newSeq = append(newSeq, layerBlock...)
		seq = newSeq
	}

	fmt.Println(len(seq))
	var sb strings.Builder
	for i, v := range seq {
		sb.WriteString(strconv.Itoa(v))
		if i < len(seq)-1 {
			sb.WriteString(" ")
		}
	}
	fmt.Println(sb.String())
}


func main() {
	var scanner *bufio.Scanner
	var writer *bufio.Writer

	// Fast I/O
	scanner = bufio.NewScanner(os.Stdin)
	writer = bufio.NewWriter(os.Stdout)
	buf := make([]byte, 1024*1024)
	scanner.Buffer(buf, 1024*1024)

	defer writer.Flush()
	
	scanner.Scan()
	t, _ := strconv.Atoi(scanner.Text())
	for i := 0; i < t; i++ {
		scanner.Scan()
		parts := strings.Fields(scanner.Text())
		n, _ := strconv.Atoi(parts[0])
		m, _ := strconv.Atoi(parts[1])
		
		solver := NewSolver(n, m)
		for j := 0; j < m; j++ {
			scanner.Scan()
			parts := strings.Fields(scanner.Text())
			a, _ := strconv.Atoi(parts[0])
			b, _ := strconv.Atoi(parts[1])
			solver.AddEdge(a, b)
		}
		solver.Solve()
	}
}