package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

var (
	// ReadString returns a WORD string.
	ReadString func() string
)

func init() {
	ReadString = newReadString(os.Stdin, bufio.ScanWords)
}

func newReadString(ior io.Reader, sf bufio.SplitFunc) func() string {
	r := bufio.NewScanner(ior)
	r.Buffer(make([]byte, 1024), int(1e+9)) // for Codeforces
	r.Split(sf)

	return func() string {
		if !r.Scan() {
			panic("Scan failed")
		}
		return r.Text()
	}
}

func readInt() int {
	n, _ := strconv.Atoi(ReadString())
	return n
}

// 10 11 12 => [10, 11, 12]
func readIntSlice(size int) []int {
	a := make([]int, size)
	for i := 0; i < size; i++ {
		a[i] = readInt()
	}
	return a
}

func get2byte(size int) [][]byte {
	a := make([][]byte, size)
	for i := 0; i < size; i++ {
		var low string
		fmt.Scan(&low)
		a[i] = []byte(low)
	}
	return a
}

func transpose(a [][]int) {
	n := len(a)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			a[i][j], a[j][i] = a[j][i], a[i][j]
		}
	}
}

func fact(n, m int) int {
	res := 1
	for i := m + 1; i <= n; i++ {
		res *= i
	}
	return res
}

func perm(n, r int) int {
	if r > n {
		return 0
	}
	return fact(n, n-r)
}

func comb(n, m int) int {
	if m > n {
		return 0
	}
	return fact(n, n-m) / fact(m, 0)
}

func reverse(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func min(integers ...int) int {
	m := integers[0]
	for i, integer := range integers {
		if i == 0 {
			continue
		}
		if m > integer {
			m = integer
		}
	}
	return m
}

func max(integers ...int) int {
	m := integers[0]
	for i, integer := range integers {
		if i == 0 {
			continue
		}
		if m < integer {
			m = integer
		}
	}
	return m
}

func main() {
	h, w, k := readInt(), readInt(), readInt()
	var cs []string
	for i := 0; i < h; i++ {
		cs = append(cs, ReadString())
	}
	var res [][]int
	whiteRow := strings.Repeat(".", w)
	for hbit := 0; hbit < 1<<uint64(h); hbit++ {
		for wbit := 0; wbit < 1<<uint64(w); wbit++ {
			tmp := make([]string, len(cs))
			copy(tmp, cs)
			for i := 0; i < h; i++ {
				for j := 0; j < w; j++ {
					if (hbit>>uint64(i))&1 == 1 {
						tmp[i] = whiteRow
					}
					if (wbit>>uint64(j))&1 == 1 {
						for k := 0; k < h; k++ {
							out := []byte(tmp[k])
							out[j] = '.'
							tmp[k] = string(out)
						}
					}
				}
			}
			if calcScore(tmp) == k {
				res = append(res, []int{hbit, wbit})
			}
		}
	}
	fmt.Println(len(res))
}

func calcScore(cs []string) int {
	var counter int
	for _, strs := range cs {
		for _, c := range strs {
			if c == '#' {
				counter++
			}
		}
	}
	return counter
}
