package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	nextInt64 := func() int64 {
		scanner.Scan()
		i, _ := strconv.ParseInt(scanner.Text(), 10, 64)
		return i
	}

	N := int(nextInt64())
	if N == 0 {
		fmt.Println("0 0")
		return
	}

	type tempEvent struct {
		x, t int64
	}
	tempEvents := make([]tempEvent, N)
	for i := 0; i < N; i++ {
		tempEvents[i].x = nextInt64()
		tempEvents[i].t = nextInt64()
	}
	V := nextInt64()

	type Event struct {
		x, t, u, v int64
		cv         int
	}

	events := make([]Event, N)
	allV := make([]int64, N)
	for i := 0; i < N; i++ {
		x := tempEvents[i].x
		t := tempEvents[i].t
		events[i] = Event{
			x: x, t: t,
			u: x + V*t,
			v: x - V*t,
		}
		allV[i] = events[i].v
	}

	sort.Slice(allV, func(i, j int) bool { return allV[i] < allV[j] })
	uniqueV := make([]int64, 0, N)
	if N > 0 {
		uniqueV = append(uniqueV, allV[0])
		for i := 1; i < N; i++ {
			if allV[i] != allV[i-1] {
				uniqueV = append(uniqueV, allV[i])
			}
		}
	}

	for i := range events {
		events[i].cv = sort.Search(len(uniqueV), func(k int) bool { return uniqueV[k] >= events[i].v })
	}

	sort.Slice(events, func(i, j int) bool {
		if events[i].u != events[j].u {
			return events[i].u < events[j].u
		}
		return events[i].v < events[j].v
	})

	numUniqueV := len(uniqueV)
	st1 := NewSegTree(numUniqueV)
	st2 := NewSegTree(numUniqueV)

	maxLen1, maxLen2 := 0, 0

	for _, e := range events {
		maxPrevDP1 := st1.Query(e.cv, numUniqueV-1)
		maxPrevDP2 := st2.Query(e.cv, numUniqueV-1)

		dp2 := 1 + maxPrevDP2

		dp1 := 0
		if maxPrevDP1 > 0 {
			dp1 = 1 + maxPrevDP1
		}

		if e.u >= 0 && e.v <= 0 {
			dp1 = max(dp1, 1)
		}

		if dp1 > 0 {
			st1.Update(e.cv, dp1)
		}
		st2.Update(e.cv, dp2)

		maxLen1 = max(maxLen1, dp1)
		maxLen2 = max(maxLen2, dp2)
	}

	fmt.Printf("%d %d\n", maxLen1, maxLen2)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

type SegTree struct {
	tree []int
	size int
}

func NewSegTree(n int) *SegTree {
	if n == 0 {
		return &SegTree{tree: nil, size: 0}
	}
	return &SegTree{
		tree: make([]int, 4*n),
		size: n,
	}
}

func (st *SegTree) update(node, start, end, pos, val int) {
	if start == end {
		st.tree[node] = max(st.tree[node], val)
		return
	}
	mid := start + (end-start)/2
	if pos <= mid {
		st.update(2*node+1, start, mid, pos, val)
	} else {
		st.update(2*node+2, mid+1, end, pos, val)
	}
	st.tree[node] = max(st.tree[2*node+1], st.tree[2*node+2])
}

func (st *SegTree) query(node, start, end, l, r int) int {
	if r < start || end < l || start > end {
		return 0
	}
	if l <= start && end <= r {
		return st.tree[node]
	}
	mid := start + (end-start)/2
	p1 := st.query(2*node+1, start, mid, l, r)
	p2 := st.query(2*node+2, mid+1, end, l, r)
	return max(p1, p2)
}

func (st *SegTree) Update(pos, val int) {
	if st.size == 0 {
		return
	}
	st.update(0, 0, st.size-1, pos, val)
}

func (st *SegTree) Query(l, r int) int {
	if st.size == 0 || l > r {
		return 0
	}
	return st.query(0, 0, st.size-1, l, r)
}