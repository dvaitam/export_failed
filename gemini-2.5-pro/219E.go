package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strconv"
)

type Gap struct {
	start, end int
	score      int
	bestSpot   int
	index      int
}

type PriorityQueue []*Gap

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	if pq[i].score != pq[j].score {
		return pq[i].score > pq[j].score
	}
	return pq[i].bestSpot < pq[j].bestSpot
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Gap)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func makeGap(start, end, n int) *Gap {
	if start >= end-1 {
		return nil
	}
	g := &Gap{start: start, end: end}
	if start == 0 && end == n+1 {
		g.score = n
		g.bestSpot = 1
	} else if start == 0 {
		g.score = end - 1
		g.bestSpot = 1
	} else if end == n+1 {
		g.score = n - start
		g.bestSpot = n
	} else {
		g.score = (end - start) / 2
		g.bestSpot = (start + end) / 2
	}
	return g
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	writer := bufio.NewWriter(os.Stdout)
	defer writer.Flush()

	readInt := func() int {
		scanner.Scan()
		i, _ := strconv.Atoi(scanner.Text())
		return i
	}

	n, m := readInt(), readInt()

	carToSpot := make(map[int]int)
	startsAt := make(map[int]*Gap)
	endsAt := make(map[int]*Gap)
	pq := make(PriorityQueue, 0, m+1)

	if n > 0 {
		initialGap := makeGap(0, n+1, n)
		if initialGap != nil {
			heap.Push(&pq, initialGap)
			startsAt[0] = initialGap
			endsAt[n+1] = initialGap
		}
	}

	for k := 0; k < m; k++ {
		t, id := readInt(), readInt()

		if t == 1 {
			bestGap := heap.Pop(&pq).(*Gap)
			p := bestGap.bestSpot

			fmt.Fprintln(writer, p)
			carToSpot[id] = p

			L, R := bestGap.start, bestGap.end
			delete(startsAt, L)
			delete(endsAt, R)

			if leftG := makeGap(L, p, n); leftG != nil {
				heap.Push(&pq, leftG)
				startsAt[L] = leftG
				endsAt[p] = leftG
			}

			if rightG := makeGap(p, R, n); rightG != nil {
				heap.Push(&pq, rightG)
				startsAt[p] = rightG
				endsAt[R] = rightG
			}

		} else {
			p := carToSpot[id]
			delete(carToSpot, id)

			leftG, hasLeft := endsAt[p]
			rightG, hasRight := startsAt[p]

			L, R := 0, n+1

			if hasLeft {
				L = leftG.start
				heap.Remove(&pq, leftG.index)
				delete(startsAt, L)
				delete(endsAt, p)
			}

			if hasRight {
				R = rightG.end
				heap.Remove(&pq, rightG.index)
				delete(startsAt, p)
				delete(endsAt, R)
			}

			if mergedG := makeGap(L, R, n); mergedG != nil {
				heap.Push(&pq, mergedG)
				startsAt[L] = mergedG
				endsAt[R] = mergedG
			}
		}
	}
}