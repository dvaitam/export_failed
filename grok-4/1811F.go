package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	in := bufio.NewScanner(os.Stdin)
	in.Split(bufio.ScanWords)
	t := nextInt(in)
	for tt := 0; tt < t; tt++ {
		{
			n := nextInt(in)
			m := nextInt(in)
			adj := make([][]int, n+1)
			deg := make([]int, n+1)
			for i := 0; i < m; i++ {
				u := nextInt(in)
				v := nextInt(in)
				adj[u] = append(adj[u], v)
				adj[v] = append(adj[v], u)
				deg[u]++
				deg[v]++
			}
			kk := int(math.Sqrt(float64(n)))
			if kk*kk != n || kk < 3 {
				fmt.Println("NO")
				continue
			}
			k := kk
			var S []int
			count4, count2 := 0, 0
			bad := false
			for i := 1; i <= n; i++ {
				if deg[i] == 4 {
					S = append(S, i)
					count4++
				} else if deg[i] == 2 {
					count2++
				} else {
					bad = true
				}
			}
			if bad || count4 != k || count2 != n-k {
				fmt.Println("NO")
				continue
			}
			isS := make(map[int]bool)
			for _, x := range S {
				isS[x] = true
			}
			bad = false
			for _, v := range S {
				ds := 0
				for _, nei := range adj[v] {
					if isS[nei] {
						ds++
					}
				}
				if ds != 2 {
					bad = true
					break
				}
			}
			if bad {
				fmt.Println("NO")
				continue
			}
			idx := make([]int, n+1)
			for i := 0; i <= n; i++ {
				idx[i] = -1
			}
			for i, v := range S {
				idx[v] = i
			}
			adjS := make([][]int, k)
			for i := 0; i < k; i++ {
				v := S[i]
				for _, nei := range adj[v] {
					if isS[nei] {
						adjS[i] = append(adjS[i], idx[nei])
					}
				}
			}
			vis := make([]bool, k)
			q := []int{0}
			vis[0] = true
			cnt := 1
			for len(q) > 0 {
				cur := q[0]
				q = q[1:]
				for _, nei := range adjS[cur] {
					if !vis[nei] {
						vis[nei] = true
						cnt++
						q = append(q, nei)
					}
				}
			}
			if cnt != k {
				fmt.Println("NO")
				continue
			}
			visited := make([]bool, n+1)
			goodPaths := 0
			bad = false
			for u := 1; u <= n && !bad; u++ {
				if visited[u] || isS[u] {
					continue
				}
				var comp []int
				q = []int{u}
				visited[u] = true
				for len(q) > 0 {
					cur := q[0]
					q = q[1:]
					comp = append(comp, cur)
					for _, nei := range adj[cur] {
						if !isS[nei] && !visited[nei] {
							visited[nei] = true
							q = append(q, nei)
						}
					}
				}
				mm := len(comp)
				if mm != k-1 {
					bad = true
					continue
				}
				count0, count1, count2 := 0, 0, 0
				var ends []int
				for _, v := range comp {
					dt := 0
					for _, nei := range adj[v] {
						if !isS[nei] {
							dt++
						}
					}
					if dt == 0 {
						count0++
					} else if dt == 1 {
						count1++
						ends = append(ends, v)
					} else if dt == 2 {
						count2++
					} else {
						bad = true
						break
					}
				}
				if bad {
					continue
				}
				if count1 != 2 || count0 != 0 || count2 != mm-2 {
					bad = true
					continue
				}
				e1 := ends[0]
				e2 := ends[1]
				var s1, s2 int
				for _, nei := range adj[e1] {
					if isS[nei] {
						s1 = nei
						break
					}
				}
				for _, nei := range adj[e2] {
					if isS[nei] {
						s2 = nei
						break
					}
				}
				if s1 == 0 || s2 == 0 || s1 != s2 {
					bad = true
					continue
				}
				goodPaths++
			}
			if bad || goodPaths != k {
				fmt.Println("NO")
			} else {
				fmt.Println("YES")
			}
		}
	}
}

func nextInt(in *bufio.Scanner) int {
	in.Scan()
	r, _ := strconv.Atoi(in.Text())
	return r
}