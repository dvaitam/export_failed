package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

const MAX_SQRT_N = 640
const MAX_K = 2*MAX_SQRT_N + 5
const NTT_SIZE = 2048

var m int64
var part_le [MAX_K + 1][MAX_SQRT_N + 1]int64
var conv [MAX_SQRT_N + 1][NTT_SIZE]int64

type pair struct {
	h, w int
}

func solve_u1(t int, in *bufio.Reader, out *bufio.Writer) {
	for i := 0; i < t; i++ {
		var n int
		fmt.Fscan(in, &n)

		h_best, w_best := 1, n
		p_best := 2 * (1 + n)

		s_float := math.Sqrt(float64(n))
		s_int := int(s_float)

		for h_cand := s_int - 5; h_cand <= s_int+5; h_cand++ {
			if h_cand <= 0 {
				continue
			}
			w_cand := (n + h_cand - 1) / h_cand
			p_cand := 2 * (h_cand + w_cand)
			if p_cand < p_best {
				p_best = p_cand
				h_best, w_best = h_cand, w_cand
			} else if p_cand == p_best {
				if h_cand*w_cand < h_best*w_best {
					h_best, w_best = h_cand, w_cand
				}
			}
		}

		h, w := h_best, w_best
		if h > w {
			h, w = w, h
		}

		fmt.Fprintln(out, h, w)
		grid := make([][]byte, h)
		for r := 0; r < h; r++ {
			grid[r] = make([]byte, w)
			for c := 0; c < w; c++ {
				grid[r][c] = '.'
			}
		}

		for j := 0; j < n; j++ {
			grid[j%h][j/h] = '#'
		}

		for r := 0; r < h; r++ {
			fmt.Fprintln(out, string(grid[r]))
		}
	}
}

func power(a int64, b int64) int64 {
	res := int64(1)
	a %= m
	for b > 0 {
		if b%2 == 1 {
			res = (res * a) % m
		}
		a = (a * a) % m
		b /= 2
	}
	return res
}

func inv(n int64) int64 {
	return power(n, m-2)
}

func ntt(a []int64, invert bool) {
	n := len(a)
	for i := 1, j := 0; i < n; i++ {
		bit := n >> 1
		for ; (j & bit) != 0; bit >>= 1 {
			j ^= bit
		}
		j ^= bit
		if i < j {
			a[i], a[j] = a[j], a[i]
		}
	}

	for length := 2; length <= n; length <<= 1 {
		root_base := power(3, (m-1)/int64(length))
		if invert {
			root_base = inv(root_base)
		}
		for i := 0; i < n; i += length {
			w := int64(1)
			for j := 0; j < length/2; j++ {
				u := a[i+j]
				v := (a[i+j+length/2] * w) % m
				a[i+j] = (u + v) % m
				a[i+j+length/2] = (u - v + m) % m
				w = (w * root_base) % m
			}
		}
	}

	if invert {
		n_inv := inv(int64(n))
		for i := 0; i < n; i++ {
			a[i] = (a[i] * n_inv) % m
		}
	}
}

func precompute_u2() {
	for j := 0; j <= MAX_SQRT_N; j++ {
		part_le[0][j] = 1
	}

	for i := 1; i <= MAX_K; i++ {
		part_le[i][0] = 0
		for j := 1; j <= MAX_SQRT_N; j++ {
			part_le[i][j] = part_le[i][j-1]
			if i >= j {
				part_le[i][j] = (part_le[i][j] + part_le[i-j][j]) % m
			}
		}
	}

	for h := 0; h <= MAX_SQRT_N; h++ {
		poly := make([]int64, NTT_SIZE)
		for i := 0; i <= MAX_K; i++ {
			poly[i] = part_le[i][h]
		}
		ntt(poly, false)
		for i := 0; i < NTT_SIZE; i++ {
			poly[i] = (poly[i] * poly[i]) % m
		}
		ntt(poly, true)
		for i := 0; i < NTT_SIZE; i++ {
			conv[h][i] = poly[i]
		}
	}
}

func calc_ways(h, w, k int) int64 {
	if k < 0 {
		return 0
	}
	h_ := h - 1
	w_ := w - 1
	if h_ < 0 || w_ < 0 {
		return 0
	}

	if h_ > MAX_SQRT_N || k > MAX_K {
		return 0
	}

	N_k := part_le[k][h_]
	if k-w_-1 >= 0 {
		N_k = (N_k - part_le[k-w_-1][h_] + m) % m
	}

	poly_sq_k := conv[h_][k]

	A2_k := poly_sq_k
	if k-w_-1 >= 0 {
		A2_k = (A2_k - (2*conv[h_][k-w_-1])%m + m) % m
	}
	if k-2*w_-2 >= 0 {
		A2_k = (A2_k + conv[h_][k-2*w_-2]) % m
	}

	N0 := int64(1)
	S_k := (A2_k - (2*N0*N_k)%m + m) % m
	S_k = (S_k * inv(2)) % m

	ways := (int64(4)*N_k + int64(6)*S_k) % m
	return ways
}

func solve_u2(t int, in *bufio.Reader, out *bufio.Writer) {
	for i := 0; i < t; i++ {
		var n int
		fmt.Fscan(in, &n)

		if n == 0 {
			fmt.Fprintln(out, 0, 1)
			continue
		}
		checked_pairs := make(map[pair]bool)

		p_min := int64(2 * (1 + n))
		s_int := int(math.Sqrt(float64(n)))

		for h_cand := s_int - 5; h_cand <= s_int+5; h_cand++ {
			if h_cand <= 0 {
				continue
			}
			w_cand := (n + h_cand - 1) / h_cand
			p_cand := int64(2 * (h_cand + w_cand))
			if p_cand < p_min {
				p_min = p_cand
			}
		}

		c := int64(0)
		S := p_min / 2

		for h_cand := 1; int64(h_cand) < S; h_cand++ {
			w_cand := S - int64(h_cand)
			if int64(h_cand)*w_cand < int64(n) {
				continue
			}

			h, w := h_cand, int(w_cand)
			if h > w {
				h, w = w, h
			}

			if checked_pairs[pair{h, w}] {
				continue
			}
			checked_pairs[pair{h, w}] = true

			k := h*w - n

			if k == 0 {
				if h == w {
					c = (c + 1) % m
				} else {
					c = (c + 2) % m
				}
				continue
			}

			ways_hw := calc_ways(h, w, k)
			c = (c + ways_hw) % m

			if h != w {
				ways_wh := calc_ways(w, h, k)
				c = (c + ways_wh) % m
			}
		}
		fmt.Fprintln(out, p_min, c)
	}
}

func main() {
	in := bufio.NewReader(os.Stdin)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()

	var t, u int
	fmt.Fscan(in, &t, &u)

	if u == 1 {
		solve_u1(t, in, out)
	} else {
		fmt.Fscan(in, &m)
		if m == 2 {
			solve_u1(t, in, out)
			return
		}
		precompute_u2()
		solve_u2(t, in, out)
	}
}