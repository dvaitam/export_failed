package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	n, _ := strconv.Atoi(scanner.Text())

	usedAlbums := make(map[string]int)
	for i := 0; i < n; i++ {
		scanner.Scan()
		parts := strings.Fields(scanner.Text())
		name := parts[0]
		year, _ := strconv.Atoi(parts[1])
		usedAlbums[name] = year
	}

	scanner.Scan()
	m, _ := strconv.Atoi(scanner.Text())

	bestName := ""
	bestYear := 2012

	for i := 0; i < m; i++ {
		scanner.Scan()
		currentName := scanner.Text()

		var currentYear int
		year, found := usedAlbums[currentName]
		if !found {
			currentYear = 1899
		} else {
			currentYear = year
		}

		if currentYear < bestYear {
			bestYear = currentYear
			bestName = currentName
		} else if currentYear == bestYear {
			if currentName > bestName {
				bestName = currentName
			}
		}
	}

	fmt.Println(bestName)
}