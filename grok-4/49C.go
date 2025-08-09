package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	n, _ := strconv.Atoi(scanner.Text())

	if n == 1 {
		fmt.Println(1)
		return
	}

	writer := bufio.NewWriter(os.Stdout)
	for i := 2; i <= n; i++ {
		fmt.Fprintf(writer, "%d ", i)
	}
	fmt.Fprintf(writer, "1\n")
	writer.Flush()
}