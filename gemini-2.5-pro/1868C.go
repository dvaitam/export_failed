package main

import (
	"bufio"
	"fmt"
	"os"
)

const MOD = 998244353

type Result struct {
	s1, s2 int64
}

var memo map[uint64]Result
var n uint64
var m int64
var x int64

func power(base, exp int64) int64 {
	res := int64(1)
	base %= MOD
	for exp > 0 {
		if exp%2 == 1 {
			res = (res * base) % MOD
		}
		base = (base * base) % MOD
		exp /= 2
	}
	return res
}

func modInverse(n int64) int64 {
	return power(n, MOD-2)
}

func solveDP(c uint64) Result {
	if c == 0 {
		return Result{0, 0}
	}
	if res, ok := memo[c]; ok {
		return res
	}

	cL := c / 2
	var cR uint64
	if c > 0 {
		cR = (c - 1) / 2
	}

	resL := solveDP(cL)
	resR := solveDP(cR)

	s1 := (1 + (x*resL.s1)%MOD + (x*resR.s1)%MOD) % MOD
	if s1 < 0 {
		s1 += MOD
	}

	termL := (1 + (x*resL.s1)%MOD) % MOD
	if termL < 0 {
		termL += MOD
	}
	termR := (1 + (x*resR.s1)%MOD) % MOD
	if termR < 0 {
		termR += MOD
	}

	lca1_contrib := (x * termL) % MOD
	lca1_contrib = (lca1_contrib * termR) % MOD

	s2 := (resL.s2 + resR.s2 + lca1_contrib) % MOD
	if s2 < 0 {
		s2 += MOD
	}

	memo[c] = Result{s1, s2}
	return memo[c]
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var t int
	fmt.Fscan(in, &t)
	for i := 0; i < t; i++ {
		var nStr string
		var mInt int64
		fmt.Fscan(in, &nStr, &mInt)

		var nUint uint64
		for _, char := range nStr {
			nUint = nUint*10 + uint64(char-'0')
		}

		n = nUint
		m = mInt

		var nMod int64
		{
			temp_n := n
			nMod = int64(temp_n % MOD)
		}

		nPairs := (nMod * (nMod + 1)) % MOD
		inv2 := modInverse(2)
		nPairs = (nPairs * inv2) % MOD

		var nExp uint64
		if n == 0 {
			nExp = 0
		} else {
			nExp = n % (MOD - 1)
			if nExp == 0 {
				nExp = MOD - 1
			}
		}

		mPowN := power(m, int64(nExp))
		mPowNPlus1 := (mPowN * m) % MOD

		term1 := (nPairs * mPowNPlus1) % MOD

		var sumHUnord int64 = 0
		modInvM := modInverse(m)

		for k := int64(1); k < m; k++ {
			memo = make(map[uint64]Result)
			x = (k * modInvM) % MOD

			res := solveDP(n)
			hPrime := res.s2

			hUnordNum := (hPrime + (nMod*x)%MOD) % MOD
			if hUnordNum < 0 {
				hUnordNum += MOD
			}
			hUnord := (hUnordNum * inv2) % MOD
			sumHUnord = (sumHUnord + hUnord) % MOD
		}

		term2 := (mPowN * sumHUnord) % MOD

		ans := (term1 - term2 + MOD) % MOD
		fmt.Fprintln(out, ans)
	}
}