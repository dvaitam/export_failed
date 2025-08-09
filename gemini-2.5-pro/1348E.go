package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var scanner = bufio.NewScanner(os.Stdin)

func nextInt() int64 {
	scanner.Scan()
	i, _ := strconv.ParseInt(scanner.Text(), 10, 64)
	return i
}

func main() {
	scanner.Split(bufio.ScanWords)

	n := nextInt()
	k := nextInt()

	var totalRed int64 = 0
	var totalBlue int64 = 0
	shrubs := make([][2]int64, n)

	for i := int64(0); i < n; i++ {
		a := nextInt()
		b := nextInt()
		shrubs[i][0] = a
		shrubs[i][1] = b
		totalRed += a
		totalBlue += b
	}

	totalBaskets := (totalRed + totalBlue) / k
	if totalBaskets == 0 {
		fmt.Println(0)
		return
	}

	dp := make([]bool, k)
	dp[0] = true

	for i := int64(0); i < n; i++ {
		a := shrubs[i][0]
		b := shrubs[i][1]
		
		nextDp := make([]bool, k)

		possibleLocalRedRem := make(map[int64]bool)
		
		for r := int64(0); r < k; r++ {
			if r <= a && (k-r)%k <= b {
				possibleLocalRedRem[r] = true
			}
		}

		for prevRem := int64(0); prevRem < k; prevRem++ {
			if dp[prevRem] {
				for remX := range possibleLocalRedRem {
					newRem := (prevRem + remX) % k
					nextDp[newRem] = true
				}
			}
		}
		dp = nextDp
	}

	totalRemR := totalRed % k
	totalRemB := totalBlue % k

	canBePerfect := false
	for remX := int64(0); remX < k; remX++ {
		if dp[remX] {
			remY := (k - remX) % k
			
			remRPool := (totalRemR - remX + k) % k
			remBPool := (totalRemB - remY + k) % k

			if remRPool + remBPool < k {
				canBePerfect = true
				break
			}
		}
	}

	if canBePerfect {
		fmt.Println(totalBaskets)
	} else {
		fmt.Println(totalBaskets - 1)
	}
}