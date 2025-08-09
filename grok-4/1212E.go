package main

import (
	"fmt"
	"sort"
)

type Request struct {
	idx int
	c   int
	p   int
}

func main() {
	var n int
	fmt.Scan(&n)
	reqs := make([]Request, n)
	for i := 0; i < n; i++ {
		var c, p int
		fmt.Scan(&c, &p)
		reqs[i] = Request{i + 1, c, p}
	}
	var k int
	fmt.Scan(&k)
	r := make([]int, k+1)
	for i := 1; i <= k; i++ {
		fmt.Scan(&r[i])
	}
	// sort reqs by p descending
	sort.Slice(reqs, func(i, j int) bool {
		if reqs[i].p != reqs[j].p {
			return reqs[i].p > reqs[j].p
		}
		return reqs[i].idx < reqs[j].idx // optional
	})
	used := make([]bool, k+1)
	type Assign struct {
		req int
		tab int
	}
	assigns := []Assign{}
	totsum := 0
	for _, req := range reqs {
		// find best table: min r >= req.c, among not used, and min r, then min idx
		best_r := 1000000
		best_j := -1
		for j := 1; j <= k; j++ {
			if !used[j] && r[j] >= req.c && (r[j] < best_r || (r[j] == best_r && j < best_j)) {
				best_r = r[j]
				best_j = j
			}
		}
		if best_j != -1 {
			used[best_j] = true
			assigns = append(assigns, Assign{req.idx, best_j})
			totsum += req.p
		}
	}
	// now output
	fmt.Println(len(assigns), totsum)
	// sort assigns by req asc
	sort.Slice(assigns, func(i, j int) bool {
		return assigns[i].req < assigns[j].req
	})
	for _, as := range assigns {
		fmt.Println(as.req, as.tab)
	}
}