package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	in := bufio.NewReader(os.Stdin)
	var n int
	if _, err := fmt.Fscan(in, &n); err != nil {
		return
	}
	if n%2 == 0 {
		fmt.Println("-1")
		return
	}

	out := bufio.NewWriter(os.Stdout)
	// permutation a
	for i := 0; i < n; i++ {
		if i > 0 {
			fmt.Fprint(out, " ")
		}
		fmt.Fprint(out, i)
	}
	fmt.Fprintln(out)

	// permutation b
	for i := 0; i < n; i++ {
		if i > 0 {
			fmt.Fprint(out, " ")
		}
		fmt.Fprint(out, (i+1)%n)
	}
	fmt.Fprintln(out)

	// permutation c = (a + b) mod n
	for i := 0; i < n; i++ {
		if i > 0 {
			fmt.Fprint(out, " ")
		}
		fmt.Fprint(out, (2*i+1)%n)
	}
	fmt.Fprintln(out)
	out.Flush()
}