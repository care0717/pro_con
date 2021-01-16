package main

import (
	"bufio"
	"errors"
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

// 初期化にo(n)
// Combinationの計算はo(1)
type ModComb struct {
	mod     int
	fact    []int
	inv     []int
	factInv []int
}

func NewModComb(n, mod int) ModComb {
	if n <= 0 {
		n = 1
	}
	initFact := make([]int, n+1)
	initFactInv := make([]int, n+1)
	initInv := make([]int, n+1)
	initFact[0], initFact[1] = 1, 1
	initFactInv[0], initFactInv[1] = 1, 1
	initInv[1] = 1
	for i := 2; i <= n; i++ {
		initFact[i] = initFact[i-1] * i % mod
		initInv[i] = mod - (initInv[mod%i] * (mod / i) % mod)
		initFactInv[i] = (initFactInv[i-1] * initInv[i]) % mod
	}

	return ModComb{mod: mod, fact: initFact, inv: initInv, factInv: initFactInv}
}
func (m ModComb) Comb(n, k int) int {
	if n < k {
		return 0
	}
	if n < 0 || k < 0 {
		return 0
	}
	return m.fact[n] * (m.factInv[k] * m.factInv[n-k] % m.mod) % m.mod
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

func primeFactorize(n int) []struct {
	Prime int
	Index int
} {
	if n <= 1 {
		return nil
	}
	maxPrime := int(math.Sqrt(float64(n)))
	var result []struct {
		Prime int
		Index int
	}
	tmp := n
	for p := 2; p <= maxPrime; p++ {
		if tmp%p == 0 {
			index := 1
			tmp = tmp / p
			for tmp%p == 0 {
				index++
				tmp /= p
			}
			result = append(result, struct {
				Prime int
				Index int
			}{p, index})
		}
	}
	if tmp > 1 {
		result = append(result, struct {
			Prime int
			Index int
		}{tmp, 1})
	}
	return result
}

type UnionFind struct {
	parent []int
	rank   []int
	size   []int
}

func NewUnionFind(size int) *UnionFind {
	initParent := make([]int, size)
	initRank := make([]int, size)
	initSize := make([]int, size)
	for i := 0; i < size; i++ {
		initParent[i] = i
		initSize[i] = 1
	}
	return &UnionFind{parent: initParent, rank: initRank, size: initSize}
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

	if u.rank[rootX] <= u.rank[rootY] {
		u.parent[rootX] = rootY
		if u.rank[rootX] == u.rank[rootY] {
			u.rank[rootX]++
		}
	} else {
		u.parent[rootY] = rootX
		rootX, rootY = rootY, rootX

	}
	u.size[rootY] += u.size[rootX]
}
func (u *UnionFind) Size(x int) int {
	return u.size[u.Root(x)]
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

// 累積和
type cumulativeSum struct {
	s []int
}

// 累積和の対象となる配列を入れる
func NewCumulativeSum(as []int) cumulativeSum {
	n := len(as)
	sum := make([]int, n+1)
	for i := 0; i < n; i++ {
		sum[i+1] = sum[i] + as[i]
	}
	return cumulativeSum{s: sum}
}

// 半開区間で入れる。すべての和がほしければa=0,b=n
func (c cumulativeSum) Get(a, b int) int {
	return c.s[b] - c.s[a]
}

func main() {

}
