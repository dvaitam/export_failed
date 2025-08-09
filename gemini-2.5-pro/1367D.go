package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func solve(reader *bufio.Reader) {
	sStr, _ := reader.ReadString('\n')
	s := strings.TrimSpace(sStr)

	mStr, _ := reader.ReadString('\n')
	m, _ := strconv.Atoi(strings.TrimSpace(mStr))

	bLine, _ := reader.ReadString('\n')
	bStrs := strings.Split(strings.TrimSpace(bLine), " ")
	b := make([]int, m)
	if m > 0 {
		for i := 0; i < m; i++ {
			b[i], _ = strconv.Atoi(bStrs[i])
		}
	}

	freq := make(map[rune]int)
	for _, char := range s {
		freq[char]++
	}

	t := make([]rune, m)
	filledCount := 0

	for filledCount < m {
		zeroIndices := []int{}
		for i := 0; i < m; i++ {
			if b[i] == 0 && t[i] == 0 {
				zeroIndices = append(zeroIndices, i)
			}
		}

		if len(zeroIndices) == 0 {
			break
		}

		k := len(zeroIndices)

		var chosenChar rune
		for char := 'z'; char >= 'a'; char-- {
			if freq[char] >= k {
				chosenChar = char
				break
			}
		}

		freq[chosenChar] -= k
		for _, idx := range zeroIndices {
			t[idx] = chosenChar
		}
		filledCount += k

		for j := 0; j < m; j++ {
			if t[j] == 0 {
				for _, i := range zeroIndices {
					b[j] -= abs(j - i)
				}
			}
		}
	}

	fmt.Println(string(t))
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	qStr, _ := reader.ReadString('\n')
	q, _ := strconv.Atoi(strings.TrimSpace(qStr))
	for i := 0; i < q; i++ {
		solve(reader)
	}
}