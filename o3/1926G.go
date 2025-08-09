package main

import (
	"bufio"
	"fmt"
	"os"
)

const (
	None = 0
	Party = 1
	Sleep = 2
)

var (
	g   [][]int
	typ []byte
	ans int
)

func dfs(v, p int) int {
	cur := None
	switch typ[v] {
	case 'P':
		cur = Party
	case 'S':
		cur = Sleep
	}
	for _, to := range g[v] {
		if to == p {
			continue
		}
		child := dfs(to, v)
		if child == None {
			continue
		}
		if cur == None {
			cur = child
		} else if cur == child {
			continue
		} else {
			ans++        // must cut the edge (v,to)
			// do not merge types
		}
	}
	return cur
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var t int
	fmt.Fscan(in, &t)
	for ; t > 0; t-- {
		var n int
		fmt.Fscan(in, &n)

		g = make([][]int, n)
		for i := 1; i < n; i++ {
			var a int
			fmt.Fscan(in, &a)
			a-- // 0-based
			g[a] = append(g[a], i)
			g[i] = append(g[i], a)
		}

		var s string
		fmt.Fscan(in, &s)
		typ = []byte(s)

		ans = 0
		dfs(0, -1)
		fmt.Fprintln(out, ans)
	}
}