package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()
	var t int
	fmt.Fscan(in, &t)
	for _ = range make([]int, t) {
		var n int
		fmt.Fscan(in, &n)
		for i := 1; i <= n; i++ {
			if i <= 2 {
				fmt.Fprintln(out, i, 1)
			} else {
				fmt.Fprintln(out, i, i)
			}
		}
	}
}