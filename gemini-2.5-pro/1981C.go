package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var reader = bufio.NewReader(os.Stdin)
var writer = bufio.NewWriter(os.Stdout)

func readLine() string {
	line, _ := reader.ReadString('\n')
	return strings.TrimSpace(line)
}

func readInts() []int64 {
	line := readLine()
	parts := strings.Fields(line)
	nums := make([]int64, len(parts))
	for i, p := range parts {
		nums[i], _ = strconv.ParseInt(p, 10, 64)
	}
	return nums
}

func solve() {
	n_int := readInts()[0]
	n := int(n_int)
	a := readInts()

	b := make([]int64, n+1)
	knownIndices := []int{}
	for i := 0; i < n; i++ {
		b[i+1] = a[i]
		if a[i] != -1 {
			knownIndices = append(knownIndices, i+1)
		}
	}

	if len(knownIndices) == 0 {
		for i := 1; i <= n; i++ {
			if i%2 == 1 {
				fmt.Fprint(writer, "1 ")
			} else {
				fmt.Fprint(writer, "2 ")
			}
		}
		fmt.Fprintln(writer)
		return
	}

	firstKnown := knownIndices[0]
	for i := firstKnown - 1; i >= 1; i-- {
		if b[i+1]%2 == 0 {
			b[i] = b[i+1] / 2
		} else {
			b[i] = b[i+1] * 2
		}
	}

	lastKnown := knownIndices[len(knownIndices)-1]
	for i := lastKnown; i < n; i++ {
		if b[i]%2 == 0 {
			b[i+1] = b[i] / 2
		} else {
			b[i+1] = b[i] * 2
		}
	}

	for k := 0; k < len(knownIndices)-1; k++ {
		startIdx := knownIndices[k]
		endIdx := knownIndices[k+1]

		if startIdx+1 == endIdx {
			if !(b[startIdx] == b[endIdx]/2 || b[endIdx] == b[startIdx]/2) {
				fmt.Fprintln(writer, -1)
				return
			}
			continue
		}

		maxVal := int64(2000000000)

		fwdMin := make([]int64, n+1)
		fwdMax := make([]int64, n+1)
		fwdMin[startIdx] = b[startIdx]
		fwdMax[startIdx] = b[startIdx]
		for i := startIdx; i < endIdx; i++ {
			fwdMin[i+1] = fwdMin[i] / 2
			if fwdMin[i+1] == 0 {
				fwdMin[i+1] = 1
			}
			fwdMax[i+1] = fwdMax[i]*2 + 1
			if fwdMax[i+1] > maxVal {
				fwdMax[i+1] = maxVal
			}
		}

		bwdMin := make([]int64, n+1)
		bwdMax := make([]int64, n+1)
		bwdMin[endIdx] = b[endIdx]
		bwdMax[endIdx] = b[endIdx]
		for i := endIdx; i > startIdx; i-- {
			bwdMin[i-1] = bwdMin[i] / 2
			if bwdMin[i-1] == 0 {
				bwdMin[i-1] = 1
			}
			bwdMax[i-1] = bwdMax[i]*2 + 1
			if bwdMax[i-1] > maxVal {
				bwdMax[i-1] = maxVal
			}
		}

		l := make([]int64, n+1)
		r := make([]int64, n+1)
		for i := startIdx; i <= endIdx; i++ {
			l[i] = fwdMin[i]
			if bwdMin[i] > l[i] {
				l[i] = bwdMin[i]
			}
			r[i] = fwdMax[i]
			if bwdMax[i] < r[i] {
				r[i] = bwdMax[i]
			}
			if l[i] > r[i] {
				fmt.Fprintln(writer, -1)
				return
			}
		}

		for i := startIdx; i < endIdx; i++ {
			curr := b[i]

			c1 := curr / 2
			if c1 > 0 && c1 >= l[i+1] && c1 <= r[i+1] {
				b[i+1] = c1
				continue
			}

			c2 := curr * 2
			if c2 > 0 && c2 <= 1000000000 && c2 >= l[i+1] && c2 <= r[i+1] {
				b[i+1] = c2
				continue
			}

			c3 := curr*2 + 1
			if c3 > 0 && c3 <= 1000000000 && c3 >= l[i+1] && c3 <= r[i+1] {
				b[i+1] = c3
				continue
			}

			fmt.Fprintln(writer, -1)
			return
		}
	}

	for i := 1; i <= n; i++ {
		fmt.Fprint(writer, b[i])
		if i < n {
			fmt.Fprint(writer, " ")
		}
	}
	fmt.Fprintln(writer)
}

func main() {
	defer writer.Flush()
	t_str := readLine()
	t, _ := strconv.Atoi(t_str)
	for i := 0; i < t; i++ {
		solve()
	}
}