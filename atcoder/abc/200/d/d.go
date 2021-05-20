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
	"strings"
)

var (
	// ReadString returns a WORD string.
	ReadString func() string
	MOD        = 1000000007
	INF        = 9223372036854775807
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

func readFloat64() float64 {
	f, _ := strconv.ParseFloat(ReadString(), 10)
	return f
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

func modPow(x, n, mod int) int {
	res := 1
	for n != 0 {
		if n%2 == 1 {
			res = res * x % mod
		}
		x = x * x % mod
		n >>= 1
	}
	return res
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

// O(sqrt(n))
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

func sqrtIntWithFloor(n int) int {
	right := 0
	// float64によって丸められて小さくなる分足しておく
	// 1000は見積もりとしては甘いので無駄は多い
	left := int(math.Sqrt(float64(n))) + 1000
	for right+1 < left {
		mid := (right + left) / 2
		if mid*mid <= n {
			right = mid
		} else {
			left = mid
		}
	}
	return right
}

func calcGcd(ints ...int) int {
	if len(ints) == 1 {
		return ints[0]
	}
	tmp := calcPairGcd(ints[0], ints[1])
	ints = append(ints[2:], tmp)
	return calcGcd(ints...)
}
func calcPairGcd(a, b int) int {
	if b == 0 {
		return a
	} else {
		return calcPairGcd(b, a%b)
	}
}
func calcLcm(ints ...int) int {
	if len(ints) == 1 {
		return ints[0]
	}
	tmp := calcPairLcm(ints[0], ints[1])
	ints = append(ints[2:], tmp)
	return calcLcm(ints...)
}
func calcPairLcm(a, b int) int {
	return a / calcPairGcd(a, b) * b
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

type SimpleItem struct {
	N int
	C int
}

func (i SimpleItem) Priority() int {
	return i.C
}
func (i SimpleItem) Cost() int {
	return i.C
}
func (i SimpleItem) Node() int {
	return i.N
}

type Item interface {
	Priority() int
	Cost() int
	Node() int
}

// Priorityの低いものが先に出てくる
type PriorityQueue struct {
	items []Item
}

func (pq *PriorityQueue) Push(item Item) {
	index := len(pq.items)
	for index > 0 {
		parentIndex := (index - 1) / 2
		if pq.items[parentIndex].Priority() <= item.Priority() {
			break
		}
		if index == len(pq.items) {
			pq.items = append(pq.items, pq.items[parentIndex])
		} else {
			pq.items[index] = pq.items[parentIndex]
		}
		index = parentIndex
	}
	if index == len(pq.items) {
		pq.items = append(pq.items, item)
	} else {
		pq.items[index] = item
	}
}
func (pq *PriorityQueue) Pop() (Item, error) {
	n := len(pq.items)
	if n == 0 {
		return nil, errors.New("PriorityQueue is empty")
	}
	popItem := pq.items[0]
	if n == 1 {
		pq.items = pq.items[:n-1]
		return popItem, nil
	}
	last := pq.items[n-1]
	pq.items = pq.items[:n-1]
	n -= 1
	index := 0
	for index*2+1 < n {
		a := index*2 + 1
		b := index*2 + 2
		if b < n && pq.items[b].Priority() < pq.items[a].Priority() {
			a = b
		}
		if pq.items[a].Priority() >= last.Priority() {
			break
		}
		pq.items[index] = pq.items[a]
		index = a
	}
	if index == n {
		pq.items = append(pq.items, last)
	} else {
		pq.items[index] = last
	}
	return popItem, nil
}
func (pq PriorityQueue) IsNotEmpty() bool {
	return len(pq.items) > 0
}
func NewPriorityQueue(items []Item) *PriorityQueue {
	pq := &PriorityQueue{}
	for _, i := range items {
		pq.Push(i)
	}
	return pq
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

func dijkstra(n int, edges []map[int]int, start int) []int {
	pq := NewPriorityQueue(nil)
	pq.Push(SimpleItem{
		N: start,
		C: 0,
	})
	visited := make([]bool, n)
	costs := make([]int, n)
	for i := 0; i < n; i++ {
		costs[i] = -1
	}
	var count int
	for pq.IsNotEmpty() {
		i, _ := pq.Pop()
		if visited[i.Node()] {
			continue
		}
		visited[i.Node()] = true
		costs[i.Node()] = i.Cost()

		for to, c := range edges[i.Node()] {
			if visited[to] {
				continue
			}
			cost := i.Cost() + c
			pq.Push(SimpleItem{
				N: to,
				C: cost,
			})
		}
		count++
	}

	return costs
}

func bitToIndex(n int) []int {
	var res []int
	var count int
	for n > 0 {
		if n%2 == 1 {
			res = append(res, count)
		}
		count++
		n /= 2
	}
	return res
}

func main() {
	n := readInt()
	as := readIntSlice(n)
	var maxPattern int
	if n > 7 {
		maxPattern = 202
	} else {
		maxPattern = int(math.Pow(2, float64(n)))
	}
	modMap := make(map[int][]int)
	for bit := 1; bit < maxPattern; bit++ {
		var sum int
		indexs := bitToIndex(bit)
		for _, i := range indexs {
			sum += as[i]
		}
		sum %= 200
		if bs, ok := modMap[sum]; ok {
			fmt.Println("Yes")
			fmt.Println(buildResult(bs))
			fmt.Println(buildResult(indexs))
			return
		} else {
			modMap[sum] = indexs
		}
	}
	fmt.Println("No")
}

func buildResult(as []int) string {
	n := len(as)
	res := make([]string, n+1)
	res[0] = strconv.Itoa(n)
	for i := 1; i <= n; i++ {
		res[i] = strconv.Itoa(as[i-1] + 1)
	}
	return strings.Join(res, " ")
}
