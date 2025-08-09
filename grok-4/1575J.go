package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	readInt := func() int {
		scanner.Scan()
		n, _ := strconv.Atoi(scanner.Text())
		return n
	}
	n := readInt()
	m := readInt()
	k := readInt()
	a := make([][]int, n)
	for i := 0; i < n; i++ {
		a[i] = make([]int, m)
		for j := 0; j < m; j++ {
			a[i][j] = readInt()
		}
	}
	c := make([]int, k)
	for i := 0; i < k; i++ {
		c[i] = readInt() - 1 // 0-based
	}
	// now simulate
	ans := make([]int, k)
	for i := 0; i < k; i++ {
		row := 0
		col := c[i]
		for row < n {
			dir := a[row][col]
			next_row := row
			next_col := col
			if dir == 1 {
				next_col++
			} else if dir == 2 {
				next_row++
			} else if dir == 3 {
				next_col--
			}
			// leave
			a[row][col] = 2
			// check if exit
			if next_row >= n {
				ans[i] = next_col + 1 // 1-based
				break
			}
			// else move
			row = next_row
			col = next_col
		}
	}
	// output
	for i := 0; i < k; i++ {
		if i > 0 {
			fmt.Print(" ")
		}
		fmt.Print(ans[i])
	}
	fmt.Println()
}