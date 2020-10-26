package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
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

type SegmentTree interface {
	Get(start, end int) int
	Update(x, val int)
	Add(x, val int)
}

type SumSegmentTree struct {
	node []int
	n    int
}

func (s SumSegmentTree) Get(start, end int) int {
	return s.sum(start, end, 0, 0, s.n)
}

func (s SumSegmentTree) sum(start, end, k, l, r int) int {
	if r <= start || end <= l {
		return 0
	}
	if start <= l && r <= end {
		return s.node[k]
	}
	return s.sum(start, end, 2*k+1, l, (l+r)/2) + s.sum(start, end, 2*k+2, (l+r)/2, r)
}

func (s *SumSegmentTree) Update(x, val int) {
	x += s.n - 1
	s.node[x] = val
	s.rebuild(x)
}
func (s *SumSegmentTree) Add(x, val int) {
	x += s.n - 1
	s.node[x] += val
	s.rebuild(x)
}
func (s *SumSegmentTree) rebuild(x int) {
	for x > 0 {
		x = (x - 1) / 2
		s.node[x] = s.node[2*x+1] + s.node[2*x+2]
	}
}
func NewSumSegmentTree(list []int) SegmentTree {
	l := len(list)
	n := 1
	for n < l {
		n *= 2
	}
	node := make([]int, 2*n-1)
	for i, n := range list {
		node[i+n-1] = n
	}
	for i := n - 2; i >= 0; i-- {
		node[i] = node[2*i+1] + node[2*i+2]
	}
	return &SumSegmentTree{node: node, n: n}
}

func main() {

}
