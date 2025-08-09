package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

const inf = math.MaxInt32

var scanner = bufio.NewScanner(os.Stdin)
var writer = bufio.NewWriter(os.Stdout)

func nextInt() int {
	scanner.Scan()
	i, _ := strconv.Atoi(scanner.Text())
	return i
}

func flush() {
	writer.Flush()
}

type Query struct {
	l, r, id int
}

type AnswerST struct {
	n    int
	tree []int
	lazy []int
}

func NewAnswerST(n int) *AnswerST {
	st := &AnswerST{
		n:    n,
		tree: make([]int, 4*n+4),
		lazy: make([]int, 4*n+4),
	}
	for i := range st.tree {
		st.tree[i] = inf
		st.lazy[i] = inf
	}
	return st
}

func (st *AnswerST) push(node int) {
	if st.lazy[node] != inf {
		st.tree[2*node] = min(st.tree[2*node], st.lazy[node])
		st.lazy[2*node] = min(st.lazy[2*node], st.lazy[node])
		st.tree[2*node+1] = min(st.tree[2*node+1], st.lazy[node])
		st.lazy[2*node+1] = min(st.lazy[2*node+1], st.lazy[node])
		st.lazy[node] = inf
	}
}

func (st *AnswerST) update(node, start, end, l, r, val int) {
	if start > end || start > r || end < l {
		return
	}
	if l <= start && end <= r {
		st.tree[node] = min(st.tree[node], val)
		st.lazy[node] = min(st.lazy[node], val)
		return
	}
	st.push(node)
	mid := (start + end) / 2
	st.update(2*node, start, mid, l, r, val)
	st.update(2*node+1, mid+1, end, l, r, val)
	st.tree[node] = min(st.tree[2*node], st.tree[2*node+1])
}

func (st *AnswerST) query(node, start, end, idx int) int {
	if start > end || start > idx || end < idx {
		return inf
	}
	if start == end {
		return st.tree[node]
	}
	st.push(node)
	mid := (start + end) / 2
	if idx <= mid {
		return st.query(2*node, start, mid, idx)
	}
	return st.query(2*node+1, mid+1, end, idx)
}

type PosST struct {
	n    int
	tree []int
}

func NewPosST(n int) *PosST {
	return &PosST{
		n:    n,
		tree: make([]int, 4*n+4),
	}
}

func (st *PosST) update(node, start, end, idx, val int) {
	if start == end {
		st.tree[node] = val
		return
	}
	mid := (start + end) / 2
	if idx <= mid {
		st.update(2*node, start, mid, idx, val)
	} else {
		st.update(2*node+1, mid+1, end, idx, val)
	}
	st.tree[node] = max(st.tree[2*node], st.tree[2*node+1])
}

func (st *PosST) query(node, start, end, l, r int) int {
	if start > end || start > r || end < l {
		return 0
	}
	if l <= start && end <= r {
		return st.tree[node]
	}
	mid := (start + end) / 2
	p1 := st.query(2*node, start, mid, l, r)
	p2 := st.query(2*node+1, mid+1, end, l, r)
	return max(p1, p2)
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	scanner.Split(bufio.ScanWords)
	n := nextInt()
	q := nextInt()

	a := make([]int, n+1)
	for i := 1; i <= n; i++ {
		a[i] = nextInt()
	}

	queriesByR := make([][]Query, n+1)
	for i := 0; i < q; i++ {
		l, r := nextInt(), nextInt()
		queriesByR[r] = append(queriesByR[r], Query{l, r, i})
	}

	ans := make([]int, q)
	answerST := NewAnswerST(n)
	posST := NewPosST(n)

	for r := 1; r <= n; r++ {
		v := a[r]

		currV := v
		for {
			j := posST.query(1, 1, n, currV+1, n)
			if j == 0 {
				break
			}
			u := a[j]
			answerST.update(1, 1, n, 1, j, u-v)
			currV = u
		}

		currV = v
		for {
			j := posST.query(1, 1, n, 1, currV-1)
			if j == 0 {
				break
			}
			u := a[j]
			answerST.update(1, 1, n, 1, j, v-u)
			currV = u
		}

		posST.update(1, 1, n, v, r)

		for _, query := range queriesByR[r] {
			ans[query.id] = answerST.query(1, 1, n, query.l)
		}
	}

	for i := 0; i < q; i++ {
		fmt.Fprintln(writer, ans[i])
	}
	flush()
}