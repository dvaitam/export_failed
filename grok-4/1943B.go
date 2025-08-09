package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

const MAXN = 200010
const LOG = 18

var (
	mod1       int64 = 1000000007
	mod2       int64 = 1000000009
	base1      int64 = 131
	base2      int64 = 137
	pow1       [MAXN]int64
	pow2       [MAXN]int64
	h1         [MAXN]int64
	h2         [MAXN]int64
	rh1        [MAXN]int64
	rh2        [MAXN]int64
	stMinWhole [LOG][MAXN]int
	stMaxWhole [LOG][MAXN]int
	stMinOdd   [LOG][MAXN]int
	stMaxOdd   [LOG][MAXN]int
	stMinEven  [LOG][MAXN]int
	stMaxEven  [LOG][MAXN]int
)

func buildSparseMin(arr []int, size int, st [LOG][MAXN]int) [LOG][MAXN]int {
	for i := 1; i <= size; i++ {
		st[0][i] = arr[i]
	}
	for k := 1; k < LOG; k++ {
		for i := 1; i+(1<<k)-1 <= size; i++ {
			st[k][i] = min(st[k-1][i], st[k-1][i+(1<<(k-1))])
		}
	}
	return st
}

func buildSparseMax(arr []int, size int, st [LOG][MAXN]int) [LOG][MAXN]int {
	for i := 1; i <= size; i++ {
		st[0][i] = arr[i]
	}
	for k := 1; k < LOG; k++ {
		for i := 1; i+(1<<k)-1 <= size; i++ {
			st[k][i] = max(st[k-1][i], st[k-1][i+(1<<(k-1))])
		}
	}
	return st
}

func querySparseMin(l, r int, st [LOG][MAXN]int) int {
	if l > r {
		return math.MaxInt32
	}
	len := r - l + 1
	k := int(math.Log2(float64(len)))
	return min(st[k][l], st[k][r-(1<<k)+1])
}

func querySparseMax(l, r int, st [LOG][MAXN]int) int {
	if l > r {
		return math.MinInt32
	}
	len := r - l + 1
	k := int(math.Log2(float64(len)))
	return max(st[k][l], st[k][r-(1<<k)+1])
}

func getHash(l, r int, h [MAXN]int64, pw [MAXN]int64, mod int64) int64 {
	mul := (h[l-1] * pw[r-l+1]) % mod
	res := (h[r] - mul + mod) % mod
	return res
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	var t int
	fmt.Fscan(in, &t)
	for tt := 0; tt < t; tt++ {
		var n, q int
		fmt.Fscan(in, &n, &q)
		var ss string
		fmt.Fscan(in, &ss)
		s := make([]byte, n+1)
		for i := 1; i <= n; i++ {
			s[i] = ss[i-1]
		}
		wholeArr := make([]int, n+1)
		for i := 1; i <= n; i++ {
			wholeArr[i] = int(s[i] - 'a')
		}
		stMinWhole = buildSparseMin(wholeArr, n, stMinWhole)
		stMaxWhole = buildSparseMax(wholeArr, n, stMaxWhole)

		sizeOdd := (n + 1) / 2
		oddArr := make([]int, sizeOdd+1)
		j := 1
		for i := 1; i <= n; i += 2 {
			oddArr[j] = int(s[i] - 'a')
			j++
		}
		stMinOdd = buildSparseMin(oddArr, sizeOdd, stMinOdd)
		stMaxOdd = buildSparseMax(oddArr, sizeOdd, stMaxOdd)

		sizeEven := n / 2
		evenArr := make([]int, sizeEven+1)
		j = 1
		for i := 2; i <= n; i += 2 {
			evenArr[j] = int(s[i] - 'a')
			j++
		}
		stMinEven = buildSparseMin(evenArr, sizeEven, stMinEven)
		stMaxEven = buildSparseMax(evenArr, sizeEven, stMaxEven)

		rs := make([]byte, n+1)
		for i := 1; i <= n; i++ {
			rs[i] = s[n+1-i]
		}

		pow1[0] = 1
		for i := 1; i <= n; i++ {
			pow1[i] = pow1[i-1] * base1 % mod1
		}
		pow2[0] = 1
		for i := 1; i <= n; i++ {
			pow2[i] = pow2[i-1] * base2 % mod2
		}

		h1[0] = 0
		for i := 1; i <= n; i++ {
			h1[i] = (h1[i-1]*base1 + int64(s[i]-'a'+1)) % mod1
		}
		h2[0] = 0
		for i := 1; i <= n; i++ {
			h2[i] = (h2[i-1]*base2 + int64(s[i]-'a'+1)) % mod2
		}

		rh1[0] = 0
		for i := 1; i <= n; i++ {
			rh1[i] = (rh1[i-1]*base1 + int64(rs[i]-'a'+1)) % mod1
		}
		rh2[0] = 0
		for i := 1; i <= n; i++ {
			rh2[i] = (rh2[i-1]*base2 + int64(rs[i]-'a'+1)) % mod2
		}

		for qq := 0; qq < q; qq++ {
			var l, r int
			fmt.Fscan(in, &l, &r)
			m := r - l + 1
			total := int64(m) * int64(m+1) / 2

			minv := querySparseMin(l, r, stMinWhole)
			maxv := querySparseMax(l, r, stMaxWhole)
			if minv == maxv {
				fmt.Fprintln(out, 0)
				continue
			}

			// check bi-constant
			subOddEqual := false
			d := l % 2
			firstP := l
			lastP := r
			if lastP%2 != d {
				lastP--
			}
			if firstP <= lastP {
				var leftIdx, rightIdx int
				var qmin, qmax int
				if d == 1 {
					leftIdx = (firstP + 1) / 2
					rightIdx = (lastP + 1) / 2
					qmin = querySparseMin(leftIdx, rightIdx, stMinOdd)
					qmax = querySparseMax(leftIdx, rightIdx, stMaxOdd)
				} else {
					leftIdx = firstP / 2
					rightIdx = lastP / 2
					qmin = querySparseMin(leftIdx, rightIdx, stMinEven)
					qmax = querySparseMax(leftIdx, rightIdx, stMaxEven)
				}
				subOddEqual = qmin == qmax
			} else {
				subOddEqual = true // empty, but shouldn't happen
			}

			subEvenEqual := false
			dEven := (l % 2 + 1) % 2
			firstPEven := l
			if firstPEven%2 != dEven {
				firstPEven++
			}
			lastPEven := r
			if lastPEven%2 != dEven {
				lastPEven--
			}
			if firstPEven <= lastPEven {
				var leftIdx, rightIdx int
				var qmin, qmax int
				if dEven == 1 {
					leftIdx = (firstPEven + 1) / 2
					rightIdx = (lastPEven + 1) / 2
					qmin = querySparseMin(leftIdx, rightIdx, stMinOdd)
					qmax = querySparseMax(leftIdx, rightIdx, stMaxOdd)
				} else {
					leftIdx = firstPEven / 2
					rightIdx = lastPEven / 2
					qmin = querySparseMin(leftIdx, rightIdx, stMinEven)
					qmax = querySparseMax(leftIdx, rightIdx, stMaxEven)
				}
				subEvenEqual = qmin == qmax
			} else {
				subEvenEqual = true // empty?
			}

			if subOddEqual && subEvenEqual {
				numOdd := (m + 1) / 2
				sumOdd := int64(numOdd) * int64(numOdd)
				f := total - sumOdd
				fmt.Fprintln(out, f)
				continue
			}

			// else
			bad := int64(1)
			fh1v := getHash(l, r, h1, pow1, mod1)
			fh2v := getHash(l, r, h2, pow2, mod2)
			rleft := n + 1 - r
			rright := n + 1 - l
			rhv1 := getHash(rleft, rright, rh1, pow1, mod1)
			rhv2 := getHash(rleft, rright, rh2, pow2, mod2)
			if fh1v == rhv1 && fh2v == rhv2 {
				bad += int64(m)
			}
			f := total - bad
			fmt.Fprintln(out, f)
		}
	}
}