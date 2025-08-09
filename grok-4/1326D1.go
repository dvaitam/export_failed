package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const (
	mod1  = 1000000007
	mod2  = 1000000009
	base1 = 31
	base2 = 37
)

type Hash struct {
	h1, h2, pow1, pow2 []int64
	mod1, mod2, base1, base2 int64
}

func NewHash(s string, base1, base2, mod1, mod2 int64) *Hash {
	n := len(s)
	pow1 := make([]int64, n+1)
	pow2 := make([]int64, n+1)
	h1 := make([]int64, n+1)
	h2 := make([]int64, n+1)
	pow1[0] = 1
	pow2[0] = 1
	for i := 1; i <= n; i++ {
		pow1[i] = pow1[i-1] * base1 % mod1
		pow2[i] = pow2[i-1] * base2 % mod2
	}
	for i := 1; i <= n; i++ {
		h1[i] = (h1[i-1]*base1 + int64(s[i-1]-'a'+1)) % mod1
		h2[i] = (h2[i-1]*base2 + int64(s[i-1]-'a'+1)) % mod2
	}
	return &Hash{h1: h1, h2: h2, pow1: pow1, pow2: pow2, mod1: mod1, mod2: mod2, base1: base1, base2: base2}
}

func (hs *Hash) get(l, r int) (int64, int64) {
	len := r - l + 1
	v1 := (hs.h1[r+1] - hs.h1[l]*hs.pow1[len]%hs.mod1 + hs.mod1) % hs.mod1
	v2 := (hs.h2[r+1] - hs.h2[l]*hs.pow2[len]%hs.mod2 + hs.mod2) % hs.mod2
	return v1, v2
}

func reverse(s string) string {
	b := []byte(s)
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
	return string(b)
}

func main() {
	in := bufio.NewReader(os.Stdin)
	var t int
	fmt.Fscan(in, &t)
	for _t := 0; _t < t; _t++ {
		var s string
		fmt.Fscan(in, &s)
		n := len(s)
		rs := reverse(s)
		hs := NewHash(s, base1, base2, mod1, mod2)
		hrs := NewHash(rs, base1, base2, mod1, mod2)

		maxM := 0
		bestT := ""

		for k := 0; k <= n/2; k++ {
			match := true
			if k > 0 {
				l, r := n-k, n-1
				h_s1, h_s2 := hs.get(l, r)
				h_rs1, h_rs2 := hrs.get(l, r)
				match = h_s1 == h_rs1 && h_s2 == h_rs2
			}
			if !match {
				continue
			}
			maxl := n - 2*k
			if maxl < 0 {
				continue
			}

			// Case 1: prefix middle
			l1 := 0
			for ll := maxl; ll >= 0; ll-- {
				if ll == 0 {
					l1 = 0
					break
				}
				start := k
				end := k + ll - 1
				if end >= n {
					continue
				}
				h_left1, h_left2 := hs.get(start, end)
				low := n - 1 - end
				high := n - 1 - start
				h_right1, h_right2 := hrs.get(low, high)
				if h_left1 == h_right1 && h_left2 == h_right2 {
					l1 = ll
					break
				}
			}
			m1 := 2*k + l1
			if m1 > maxM {
				maxM = m1
				p := k + l1
				a := s[0:p]
				b := s[n-k : n]
				bestT = a + b
			}

			// Case 2: suffix middle
			l2 := 0
			for ll := maxl; ll >= 0; ll-- {
				if ll == 0 {
					l2 = 0
					break
				}
				start := n - k - ll
				end := n - k - 1
				if start < 0 {
					continue
				}
				h_left1, h_left2 := hs.get(start, end)
				low := n - 1 - end
				high := n - 1 - start
				h_right1, h_right2 := hrs.get(low, high)
				if h_left1 == h_right1 && h_left2 == h_right2 {
					l2 = ll
					break
				}
			}
			m2 := 2*k + l2
			if m2 > maxM {
				maxM = m2
				q := k + l2
				a := s[0:k]
				b := s[n-q : n]
				bestT = a + b
			}
		}

		// If maxM == 0, which shouldn't happen, but to be safe
		if maxM == 0 {
			bestT = s[0:1]
		}

		var out strings.Builder
		out.WriteString(bestT + "\n")
		fmt.Print(out.String())
	}
}