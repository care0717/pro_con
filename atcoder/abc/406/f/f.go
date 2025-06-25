package main

import (
	"bufio"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {

}

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

func join(xs []int) string {
	res := make([]string, len(xs))
	for i, x := range xs {
		res[i] = strconv.Itoa(x)
	}
	return strings.Join(res, " ")
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

// O(m)
func comb(n, m int) int {
	if m > n {
		return 0
	}
	res := 1
	for i := 1; i <= m; i++ {
		res *= n - i + 1
		res /= i
	}
	return res
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
	for i, v := range ints {
		ints[i] = int(math.Abs(float64(v)))
	}
	if len(ints) == 1 {
		return ints[0]
	}
	tmp := calcPairGcd(ints[0], ints[1])
	ints = append(ints[2:], tmp)
	return calcGcd(ints...)
}

// O(log(min(a,b)))
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
func (u *UnionFind) Roots() []int {
	var roots []int
	for i := 0; i < len(u.parent); i++ {
		if u.Root(i) == i {
			roots = append(roots, i)
		}
	}
	return roots
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
type CumulativeSum interface {
	Get(a, b int) int
}
type cumulativeSum struct {
	s []int
}

// 累積和の対象となる配列を入れる
func NewCumulativeSum(as []int) CumulativeSum {
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

type CumulativeSum2 interface {
	Get(x1, y1, x2, y2 int) int
}
type cumulativeSum2 struct {
	s [][]int
}

func NewCumulativeSum2(as [][]int) CumulativeSum2 {
	h, w := len(as), len(as[0])
	s := make([][]int, h+1)
	s[0] = make([]int, w+1)
	for i := 0; i < h; i++ {
		s[i+1] = make([]int, w+1)
		for j := 0; j < w; j++ {
			s[i+1][j+1] = s[i][j+1] + s[i+1][j] - s[i][j] + as[i][j]
		}
	}
	return cumulativeSum2{s: s}
}
func (c cumulativeSum2) Get(x1, y1, x2, y2 int) int {
	return c.s[x2][y2] - c.s[x1][y2] - c.s[x2][y1] + c.s[x1][y1]
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

type AvlTree struct {
	root *avlNode
}

func NewAvlTree() AvlTree {
	return AvlTree{}
}

func (t *AvlTree) Insert(v int) {
	if t.root == nil {
		t.root = &avlNode{Value: v, Height: 1}
		return
	}
	t.root.insert(v)
}

func (t AvlTree) Search(v int) bool {
	if t.root == nil {
		return false
	}
	return t.root.search(v)
}

func (t AvlTree) LowerBound(v int) (int, bool) {
	if t.root == nil {
		return 0, false
	}
	return t.root.lowerBound(v, 0, false)
}

func (t AvlTree) UpperBound(v int) (int, bool) {
	if t.root == nil {
		return 0, false
	}
	return t.root.upperBound(v, 0, false)
}

func (t AvlTree) Iter() <-chan int {
	ch := make(chan int)
	if t.root == nil {
		close(ch)
		return ch
	}
	go func() {
		t.root.iter(ch)
		close(ch)
	}()
	return ch
}

func (t AvlTree) String() string {
	if t.root == nil {
		return "null"
	}
	return t.root.string()
}

type avlNode struct {
	Height int      `json:"height"`
	Left   *avlNode `json:"left,omitempty"`
	Value  int      `json:"value"`
	Right  *avlNode `json:"right,omitempty"`
}

func (n avlNode) string() string {
	b, _ := json.Marshal(n)
	return string(b)
}

func (n avlNode) iter(ch chan<- int) {
	if n.Left != nil {
		n.Left.iter(ch)
	}
	ch <- n.Value
	if n.Right != nil {
		n.Right.iter(ch)
	}
}

func (n avlNode) search(v int) bool {
	if n.Value == v {
		return true
	} else if n.Value < v {
		if n.Right == nil {
			return false
		} else {
			return n.Right.search(v)
		}
	} else {
		if n.Left == nil {
			return false
		} else {
			return n.Left.search(v)
		}
	}
}

func (n avlNode) lowerBound(v, lower int, isLower bool) (int, bool) {
	if n.Value == v {
		return v, true
	} else if n.Value < v {
		if n.Right == nil {
			return n.Value, true
		}
		return n.Right.lowerBound(v, n.Value, true)
	} else {
		if n.Left == nil {
			return lower, isLower
		}
		return n.Left.lowerBound(v, lower, isLower)
	}
}

func (n avlNode) upperBound(v, upper int, isUpper bool) (int, bool) {
	if n.Value == v {
		return v, true
	} else if n.Value < v {
		if n.Right == nil {
			return upper, isUpper
		}
		return n.Right.upperBound(v, upper, isUpper)
	} else {
		if n.Left == nil {
			return n.Value, true
		}
		return n.Left.upperBound(v, n.Value, true)
	}
}

func (n avlNode) leftHeight() int {
	if n.Left == nil {
		return 0
	}
	return n.Left.Height
}

func (n avlNode) rightHeight() int {
	if n.Right == nil {
		return 0
	}
	return n.Right.Height
}

func (n avlNode) bias() int {
	return n.leftHeight() - n.rightHeight()
}

// rightは存在している前提で呼び出す
func (n *avlNode) rotateL() {
	newNode := avlNode{Value: n.Right.Value, Right: n.Right.Right}
	left := &avlNode{Value: n.Value, Left: n.Left, Right: n.Right.Left}
	left.updateHeight()
	newNode.Left = left
	newNode.updateHeight()
	*n = newNode
}

// leftは存在している前提で呼び出す
func (n *avlNode) rotateR() {
	newNode := avlNode{Value: n.Left.Value, Left: n.Left.Left}
	right := &avlNode{Value: n.Value, Right: n.Right, Left: n.Left.Right}
	right.updateHeight()
	newNode.Right = right
	newNode.updateHeight()
	*n = newNode
}

func (n *avlNode) updateHeight() {
	n.Height = 1 + max(n.leftHeight(), n.rightHeight())
}

func (n *avlNode) insert(v int) {
	if n.Value < v {
		if n.Right == nil {
			n.Right = &avlNode{Value: v, Height: 1}
		} else {
			n.Right.insert(v)
		}
	} else {
		if n.Left == nil {
			n.Left = &avlNode{Value: v, Height: 1}
		} else {
			n.Left.insert(v)
		}
	}
	n.balance()
}

func (n *avlNode) balance() {
	bias := n.bias()
	if bias < -1 {
		// biasが負なら必ずrightは存在する
		if n.Right.bias() > 0 {
			// rightのbiasが正なので、rightのleftが存在するのでrotateRが呼び出せる
			n.Right.rotateR()
		}
		n.rotateL()
	} else if bias > 1 {
		// 逆も同じ
		if n.Left.bias() < 0 {
			n.Left.rotateL()
		}
		n.rotateR()
	} else {
		n.updateHeight()
	}
}
