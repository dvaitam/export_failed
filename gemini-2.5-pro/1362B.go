package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var scanner *bufio.Scanner

func init() {
	scanner = bufio.NewScanner(os.Stdin)
	const maxCapacity = 1024 * 1024
	buf := make([]byte, maxCapacity)
	scanner.Buffer(buf, maxCapacity)
}

func main() {
	scanner.Scan()
	t, _ := strconv.Atoi(scanner.Text())
	for i := 0; i < t; i++ {
		solve()
	}
}

func solve() {
	scanner.Scan()
	n, _ := strconv.Atoi(scanner.Text())

	scanner.Scan()
	sStr := strings.Fields(scanner.Text())

	s := make([]int, n)
	sMap := make(map[int]bool, n)
	for i := 0; i < n; i++ {
		val, _ := strconv.Atoi(sStr[i])
		s[i] = val
		sMap[val] = true
	}

	if n <= 1 {
		fmt.Println(-1)
		return
	}

	// If a valid positive k exists, it must transform any element s_i into another element s_j.
	// Therefore, k must be of the form s_i ^ s_j.
	// We can generate all possible candidates for k by fixing one element, say s[0],
	// and XORing it with all other elements in the set.
	candidates := make([]int, 0, n-1)
	for i := 1; i < n; i++ {
		candidates = append(candidates, s[0]^s[i])
	}
	// Sort the candidates to ensure we find the smallest one first.
	sort.Ints(candidates)

	lastK := -1
	for _, k := range candidates {
		// Skip duplicates since the list is sorted.
		if k == lastK {
			continue
		}
		lastK = k

		isValid := true
		// Check if this k transforms the entire set S into itself.
		// This is true if for every element `val` in S, `val ^ k` is also in S.
		for _, val := range s {
			if _, ok := sMap[val^k]; !ok {
				isValid = false
				break
			}
		}

		if isValid {
			fmt.Println(k)
			return
		}
	}

	// If no candidate k worked, no such positive integer exists.
	fmt.Println(-1)
}