package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	var n int
	fmt.Sscan(scanner.Text(), &n)
	patterns := make([]string, n)
	for i := 0; i < n; i++ {
		scanner.Scan()
		patterns[i] = scanner.Text()
	}
	m := len(patterns[0])
	r := make([]byte, m)
	for j := 0; j < m; j++ {
		seen := make(map[byte]bool)
		for i := 0; i < n; i++ {
			c := patterns[i][j]
			if c != '?' {
				seen[c] = true
			}
		}
		sz := len(seen)
		if sz > 1 {
			r[j] = '?'
		} else if sz == 1 {
			for k := range seen {
				r[j] = k
				break
			}
		} else {
			r[j] = 'a'
		}
	}
	fmt.Println(string(r))
}