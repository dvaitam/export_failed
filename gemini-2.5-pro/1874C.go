package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

const MAXN = 5001

var C [MAXN][MAXN]float64

func precompute() {
	for i := 1; i < MAXN; i++ {
		if i%2 == 1 {
			invI := 1.0 / float64(i)
			for j := 1; j <= i; j++ {
				C[i][j] = invI
			}
		} else {
			invI := 1.0 / float64(i)
			C[i][1] = invI
			if i > 1 {
				for j := 2; j <= i; j++ {
					var term1, term2 float64
					if j-2 > 0 {
						term1 = float64(j-2) * C[i-2][j-2]
					}
					term2 = float64(i-j) * C[i-2][j-1]
					C[i][j] = (term1 + term2) * invI
				}
			}
		}
	}
}

func solve() {
	line1 := readInts()
	n, m := line1[0], line1[1]
	adj := make([][]int, n+1)
	for i := 0; i < m; i++ {
		edge := readInts()
		a, b := edge[0], edge[1]
		adj[a] = append(adj[a], b)
	}

	dp := make([]float64, n+1)
	dp[n] = 1.0

	for u := n - 1; u >= 1; u-- {
		k := len(adj[u])
		if k == 0 {
			dp[u] = 0.0
			continue
		}

		probs := make([]float64, k)
		for i, v := range adj[u] {
			probs[i] = dp[v]
		}

		sort.Sort(sort.Reverse(sort.Float64Slice(probs)))

		var totalProb float64
		for j := 0; j < k; j++ {
			totalProb += C[k][j+1] * probs[j]
		}
		dp[u] = totalProb
	}

	fmt.Printf("%.12f\n", dp[1])
}

func main() {
	precompute()
	t_str := readString()
	t, _ := strconv.Atoi(t_str)
	for i := 0; i < t; i++ {
		solve()
	}
}

var reader = bufio.NewReader(os.Stdin)

func readString() string {
	s, _ := reader.ReadString('\n')
	return strings.TrimSpace(s)
}

func readInts() []int {
	s := readString()
	parts := strings.Fields(s)
	res := make([]int, len(parts))
	for i, p := range parts {
		res[i], _ = strconv.Atoi(p)
	}
	return res
}