package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"sort"
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

func readInt64() int64 {
	n, _ := strconv.ParseInt(ReadString(), 10, 64)
	return n
}

func readInt() int {
	return int(readInt64())
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

func divisor(n int) []int {
	maxDivisor := int(math.Sqrt(float64(n)))
	divisors := make([]int, 0, maxDivisor)
	for i := 1; i <= maxDivisor; i++ {
		if n%i == 0 {
			divisors = append(divisors, i)
			if i != n/i {
				divisors = append(divisors, n/i)
			}
		}
	}
	sort.Ints(divisors)
	return divisors
}

type UnionFind struct {
	parent []int
	rank   []int
}

func NewUnionFind(size int) *UnionFind {
	initParent := make([]int, size)
	initRank := make([]int, size)
	for i := 0; i < size; i++ {
		initParent[i] = i
		initRank[i] = 0
	}
	return &UnionFind{parent: initParent, rank: initRank}
}
func (u *UnionFind) Root(x int) int {
	if u.parent[x] == x {
		return x
	} else {
		u.parent[x] = u.Root(u.parent[x])
		return u.parent[x]
	}
}
func (u *UnionFind) Same(x, y int) bool {
	return u.Root(x) == u.Root(y)
}
func (u *UnionFind) Unite(x, y int) {
	rootX := u.Root(x)
	rootY := u.Root(y)
	if rootX == rootY {
		return
	}

	if u.rank[rootX] < u.rank[rootY] {
		u.parent[rootX] = rootY
	} else {
		u.parent[rootY] = rootX
		if u.rank[rootX] == u.rank[rootY] {
			u.rank[rootX]++
		}
	}
}

type Deque interface {
	Push(x int)
	Unshift(x int)
	Empty() bool
	Pop() (int, error)
	Shift() (int, error)
}

type DequeList struct {
	prep []int
	ap   []int
}

func NewDequeList() Deque {
	return &DequeList{}
}

func (d *DequeList) Push(x int) {
	d.ap = append(d.ap, x)
}
func (d *DequeList) Unshift(x int) {
	d.prep = append(d.prep, x)
}
func (d DequeList) Empty() bool {
	return len(d.prep) == 0 && len(d.ap) == 0
}

func (d *DequeList) Pop() (int, error) {
	if d.Empty() {
		return 0, errors.New("deque is empty")
	}
	lenAp := len(d.ap)
	if lenAp > 0 {
		v := d.ap[lenAp-1]
		d.ap = d.ap[:lenAp-1]
		return v, nil
	}
	v := d.prep[0]
	d.prep = d.prep[1:]
	return v, nil
}
func (d *DequeList) Shift() (int, error) {
	if d.Empty() {
		return 0, errors.New("deque is empty")
	}
	lenPrep := len(d.prep)
	if lenPrep > 0 {
		v := d.prep[lenPrep-1]
		d.prep = d.prep[:lenPrep-1]
		return v, nil
	}
	v := d.ap[0]
	d.ap = d.ap[1:]
	return v, nil
}

func main() {

}
