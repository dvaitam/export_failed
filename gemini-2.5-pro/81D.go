package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var scanner *bufio.Scanner

func init() {
	scanner = bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
}

func nextInt() int {
	scanner.Scan()
	i, _ := strconv.Atoi(scanner.Text())
	return i
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

type Album struct {
	id    int
	count int
}

type AlbumHeap []Album

func (h AlbumHeap) Len() int { return len(h) }
func (h AlbumHeap) Less(i, j int) bool {
	if h[i].count != h[j].count {
		return h[i].count > h[j].count
	}
	return h[i].id < h[j].id
}
func (h AlbumHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *AlbumHeap) Push(x interface{}) {
	*h = append(*h, x.(Album))
}

func (h *AlbumHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	n := nextInt()
	m := nextInt()
	a := make([]int, m)
	for i := 0; i < m; i++ {
		a[i] = nextInt()
	}

	low, high := 1, n
	bestK := -1
	var bestCounts []int

	for low <= high {
		kTest := low + (high-low)/2
		if kTest == 0 {
			low = 1
			continue
		}

		possibleSum := 0
		for i := 0; i < m; i++ {
			possibleSum += min(a[i], kTest)
		}

		if possibleSum >= n {
			bestK = kTest
			tempCounts := make([]int, m)
			total := 0
			for i := 0; i < m; i++ {
				tempCounts[i] = min(a[i], kTest)
				total += tempCounts[i]
			}

			excess := total - n
			for i := 0; i < m && excess > 0; i++ {
				toReduce := min(excess, tempCounts[i])
				tempCounts[i] -= toReduce
				excess -= toReduce
			}
			bestCounts = tempCounts
			high = kTest - 1
		} else {
			low = kTest + 1
		}
	}

	if bestK == -1 || 2*bestK > n {
		fmt.Println(-1)
		return
	}

	h := &AlbumHeap{}
	heap.Init(h)
	for i := 0; i < m; i++ {
		if bestCounts[i] > 0 {
			heap.Push(h, Album{id: i + 1, count: bestCounts[i]})
		}
	}

	res := make([]int, n)
	lastAlbum := -1

	for i := 0; i < n; i++ {
		top := heap.Pop(h).(Album)

		if top.id == lastAlbum && h.Len() > 0 {
			secondTop := heap.Pop(h).(Album)
			res[i] = secondTop.id
			lastAlbum = secondTop.id
			secondTop.count--
			if secondTop.count > 0 {
				heap.Push(h, secondTop)
			}
			heap.Push(h, top)
		} else {
			res[i] = top.id
			lastAlbum = top.id
			top.count--
			if top.count > 0 {
				heap.Push(h, top)
			}
		}
	}

	if n > 1 && res[0] == res[n-1] {
		fixI := -1
		for i := 1; i < n; i++ {
			if res[i-1] != res[0] && res[i] != res[0] {
				fixI = i
				break
			}
		}

		if fixI != -1 {
			prefix := res[0:fixI]
			for j := 0; j < len(prefix)/2; j++ {
				prefix[j], prefix[len(prefix)-1-j] = prefix[len(prefix)-1-j], prefix[j]
			}
		}
	}

	var sb strings.Builder
	for i, val := range res {
		sb.WriteString(strconv.Itoa(val))
		if i < n-1 {
			sb.WriteString(" ")
		}
	}
	fmt.Println(sb.String())
}