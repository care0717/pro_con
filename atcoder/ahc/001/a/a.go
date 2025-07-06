package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"math/rand"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

type Rectangle struct {
	X1         int
	Y1         int
	X2         int
	Y2         int
	R          int
	O          OverlappedDirection
	CannotGrow bool
}

func (r Rectangle) distanceFromCenter() float64 {
	return math.Sqrt(float64((r.X1-MAX_SIDE_LENGTH/2)*(r.X1-MAX_SIDE_LENGTH/2) + (r.Y1-MAX_SIDE_LENGTH/2)*(r.Y1-MAX_SIDE_LENGTH/2)))
}

func (r Rectangle) leftAdjacent(other *Rectangle) bool {
	if (r.X1 == other.X2) &&
		(r.Y1 < other.Y2 && r.Y2 > other.Y1) {
		return true
	}
	return false
}

func (r Rectangle) rightAdjacent(other *Rectangle) bool {
	if (r.X2 == other.X1) &&
		(r.Y1 < other.Y2 && r.Y2 > other.Y1) {
		return true
	}
	return false
}
func (r Rectangle) topAdjacent(other *Rectangle) bool {
	if (r.Y1 == other.Y2) &&
		(r.X1 < other.X2 && r.X2 > other.X1) {
		return true
	}
	return false
}

func (r Rectangle) bottomAdjacent(other *Rectangle) bool {
	if (r.Y2 == other.Y1) &&
		(r.X1 < other.X2 && r.X2 > other.X1) {
		return true
	}
	return false
}

func (r Rectangle) adjacent(other *Rectangle) bool {
	return r.leftAdjacent(other) || r.rightAdjacent(other) || r.topAdjacent(other) || r.bottomAdjacent(other)
}

func (r Rectangle) areaOverR(ratio float64) bool {
	return r.area() >= r.R
}
func (r Rectangle) area() int {
	return (r.X2 - r.X1) * (r.Y2 - r.Y1)
}

func (r Rectangle) overlap(other *Rectangle) bool {
	return max(r.X1, other.X1) < min(r.X2, other.X2) && max(r.Y1, other.Y1) < min(r.Y2, other.Y2)
}

func (r Rectangle) allOverlapped() bool {
	return r.O.allOverlapped()
}

func (r Rectangle) point() float64 {
	area := r.area()
	return 1 - math.Pow(1-float64(min(area, r.R))/float64(max(area, r.R)), 2)
}

func (r *Rectangle) reset(x, y int) {
	r.X1 = x
	r.X2 = y
	r.X2 = x + 1
	r.Y2 = y + 1
	r.O = OverlappedDirection{}
	return
}
func (r *Rectangle) randomGrow(delta int) {
	switch rand.Intn(4) {
	case 0:
		switch rand.Intn(4) {
		case 0:
			r.X1 -= delta
		case 1:
			r.X2 += delta
		case 2:
			r.Y1 -= delta
		case 3:
			r.Y2 += delta
		}
	default:
		width := float64(r.X2 - r.X1)
		height := float64(r.Y2 - r.Y1) // Y2 > Y1と仮定
		if rand.Float64() > width/(width+height) {
			switch rand.Intn(2) {
			case 0:
				r.X1 -= delta
			case 1:
				r.X2 += delta
			}
		} else {
			switch rand.Intn(2) {
			case 0:
				r.Y1 -= delta
			case 1:
				r.Y2 += delta
			}
		}
	}
}

func (r *Rectangle) randomMove() {
	switch rand.Intn(4) {
	case 0:

		r.X1 -= 1
		r.X2 -= 1
	case 1:
		r.X1 += 1
		r.X2 += 1
	case 2:
		r.Y1 -= 1
		r.Y2 -= 1
	case 3:
		r.Y1 += 1
		r.Y2 += 1
	}
}

func (r *Rectangle) overarea() bool {
	return r.X1 < 0 || r.Y1 < 0 || r.X2 > MAX_SIDE_LENGTH || r.Y2 > MAX_SIDE_LENGTH
}

func (r *Rectangle) include(targetPoint []int) bool {
	return r.X1 <= targetPoint[0] && targetPoint[0] < r.X2 && r.Y1 <= targetPoint[1] && targetPoint[1] < r.Y2
}

type OverlappedDirection struct {
	x1 bool
	y1 bool
	x2 bool
	y2 bool
}

func (o OverlappedDirection) allOverlapped() bool {
	return o.x1 && o.x2 && o.y1 && o.y2
}

type State struct {
	f            io.Writer
	t            float64
	delta        int
	targetPoints [][]int
	startTime    time.Time
	rectangles   []*Rectangle
}

const (
	MAX_SIDE_LENGTH = 10000
	DEBUG_COUNT     = 20000
)

func (s State) point() float64 {
	var sum float64
	for _, r := range s.rectangles {
		sum += r.point()
	}
	return sum
}

func (s State) dump() {
	for _, r := range s.rectangles {
		io.WriteString(s.f, fmt.Sprintf("%d %d %d %d\n", r.X1, r.Y1, r.X2, r.Y2))
	}
}

func (s State) minRectangle() (int, float64) {
	var minIndex int
	minPoint := 1000000.0
	for i, r := range s.rectangles {
		if r.CannotGrow {
			continue
		}
		if r.point() < minPoint {
			minPoint = r.point()
			minIndex = i
		}
	}
	return minIndex, minPoint
}

func (s State) reset(index int) []int {
	var resetIndex []int
	rectangle := s.rectangles[index]
	for i, r := range s.rectangles {
		if rectangle.adjacent(r) {
			r.reset(s.targetPoints[i][0], s.targetPoints[i][1])
			resetIndex = append(resetIndex, i)
		}
	}
	rectangle.reset(s.targetPoints[index][0], s.targetPoints[index][1])
	return resetIndex
}

func (s State) resetDirection() {
	for _, r := range s.rectangles {
		r.O = OverlappedDirection{}
	}
}

func clone[T any](from T) T {
	var to T
	var (
		buf = new(bytes.Buffer)
		enc = json.NewEncoder(buf)
		dec = json.NewDecoder(buf)
	)
	_ = enc.Encode(from)
	_ = dec.Decode(&to)
	return to
}

// ランダムに上下左右に拡張、もしくは上下左右移動、もしくは上下左右に縮む
func (s State) randomAction(index int) {
	rectangle := s.rectangles[index]
	clonedRectangle := clone(*rectangle)
	random := rand.Intn(10)
	if random < 5 {
		clonedRectangle.randomGrow(s.delta)
	} else if random < 8 {
		clonedRectangle.randomMove()
	} else {
		clonedRectangle.randomGrow(-s.delta)
	}
	// 面積が増えてポイントが減る or エリア外に出る or ターゲットとなる点を含んでなければ終了
	if (clonedRectangle.areaOverR(1) && clonedRectangle.point() < rectangle.point()) || clonedRectangle.overarea() || !clonedRectangle.include(s.targetPoints[index]) {
		return
	}
	// 他の長方形とかぶってても終了
	for i, rec := range s.rectangles {
		if i == index {
			continue
		}
		if clonedRectangle.overlap(rec) {
			return
		}
	}
	// ポイントが上がれば採用、そうでなくても確率で採用
	prob := math.Exp((clonedRectangle.point() - rectangle.point()) / s.t)
	if prob > rand.Float64() {
		s.rectangles[index] = &clonedRectangle
	}
}

func (s State) grow(index int, ratio float64) (float64, bool) {
	rectangle := s.rectangles[index]
	if rectangle.areaOverR(ratio) || rectangle.allOverlapped() {
		return 0, false
	}
	delta := 1
	var cannotGrowLeft, cannotGrowRight, cannotGrowTop, cannotGrowBottom bool
	if rectangle.X1 < delta {
		cannotGrowLeft = true
		rectangle.O.x1 = true
	} else if !rectangle.O.x1 {
		rectangle.X1 -= delta
		for i, rec := range s.rectangles {
			if i == index {
				continue
			}
			if rectangle.overlap(rec) {
				rectangle.X1 += delta
				rectangle.O.x1 = true
				if s.targetPoints[i][0]+1 == rectangle.X1 {
					cannotGrowLeft = true
				}
				break
			}
		}
	}
	if rectangle.Y1 < delta {
		cannotGrowTop = true
		rectangle.O.y1 = true
	} else if !rectangle.O.y1 {
		rectangle.Y1 -= delta
		for i, rec := range s.rectangles {
			if i == index {
				continue
			}
			if rectangle.overlap(rec) {
				rectangle.Y1 += delta
				rectangle.O.y1 = true
				if s.targetPoints[i][1]+1 == rectangle.Y1 {
					cannotGrowTop = true
				}
				break
			}
		}
	}
	if rectangle.X2 > MAX_SIDE_LENGTH-delta {
		cannotGrowRight = true
		rectangle.O.x2 = true
	} else if !rectangle.O.x2 {
		rectangle.X2 += delta
		for i, rec := range s.rectangles {
			if i == index {
				continue
			}
			if rectangle.overlap(rec) {
				rectangle.X2 -= delta
				rectangle.O.x2 = true
				if s.targetPoints[i][0] == rectangle.X2 {
					cannotGrowRight = true
				}
				break
			}
		}
	}
	if rectangle.Y2 > MAX_SIDE_LENGTH-delta {
		cannotGrowBottom = true
		rectangle.O.y2 = true
	} else if !rectangle.O.y2 {
		rectangle.Y2 += delta
		for i, rec := range s.rectangles {
			if i == index {
				continue
			}
			if rectangle.overlap(rec) {
				rectangle.Y2 -= delta
				rectangle.O.y2 = true
				if s.targetPoints[i][1] == rectangle.Y2 {
					cannotGrowBottom = true
				}
				break
			}
		}
	}
	if cannotGrowLeft && cannotGrowTop && cannotGrowRight && cannotGrowBottom {
		rectangle.CannotGrow = true
	}
	return rectangle.point(), true
}

func solve6(state State) []*Rectangle {
	solve2(state, 1)
	adLength := len(state.rectangles)
	var counter int
	for now := time.Now().Sub(state.startTime); now < 4800*time.Millisecond; now = time.Now().Sub(state.startTime) {
		index := rand.Intn(adLength)
		state.randomAction(index)
		state.t = 1 - float64(now.Milliseconds())/4800
		state.delta = max(int(10*state.t), 1)
		if counter%DEBUG_COUNT == 0 {
			state.dump()
		}
		if counter%100000 == 0 {
			var lowScore []int
			for i, r := range state.rectangles {
				if r.point() < 0.8 {
					lowScore = append(lowScore, i)
				}
			}
			state.reset(rand.Intn(len(lowScore)))
		}
		counter++
	}
	state.resetDirection()
	solve2(state, 1)
	state.dump()
	return state.rectangles
}

func solve5(state State) []*Rectangle {
	adLength := len(state.rectangles)
	var counter int
	for now := time.Now().Sub(state.startTime); now < 4800*time.Millisecond; now = time.Now().Sub(state.startTime) {
		index := rand.Intn(adLength)
		state.randomAction(index)
		state.t = 1 - float64(now.Milliseconds())/4800
		state.delta = max(int(10*state.t), 1)
		if counter%DEBUG_COUNT == 0 {
			state.dump()
		}
		counter++
	}
	state.dump()
	return state.rectangles
}

func solve4(state State) []*Rectangle {
	solve2(state, 0.7)
	adLength := len(state.rectangles)
	var counter int
	for now := time.Now().Sub(state.startTime); now < 4800*time.Millisecond; now = time.Now().Sub(state.startTime) {
		index := rand.Intn(adLength)
		state.randomAction(index)
		if now < 2000*time.Millisecond {
			state.t = 1
		} else {
			state.t = 0.1
		}
		state.delta = 1
		if counter%DEBUG_COUNT == 0 {
			state.dump()
		}
		counter++
	}
	state.resetDirection()
	solve2(state, 1)
	state.dump()
	return state.rectangles
}

// pointが低いやつから優先的に広げたあと、ポイントが低いやつを無理やり押し広げてみる
func solve3(state State) []*Rectangle {
	solve2(state, 1)

	for time.Now().Sub(state.startTime) < 480*time.Millisecond {
		minIndex, _ := state.minRectangle()
		rec := state.rectangles[minIndex]
		rec.O = OverlappedDirection{}
		var reducedRectangles []int
		for i, r := range state.rectangles {
			if i == minIndex {
				continue
			}
			if rec.leftAdjacent(r) && state.targetPoints[i][0]+1 < r.X2 {
				r.X2 -= 1
				reducedRectangles = append(reducedRectangles, i)
			}
			if rec.rightAdjacent(r) && state.targetPoints[i][0] > r.X1 {
				r.X1 += 1
				reducedRectangles = append(reducedRectangles, i)
			}
			if rec.topAdjacent(r) && state.targetPoints[i][1]+1 < r.Y2 {
				r.Y2 -= 1
				reducedRectangles = append(reducedRectangles, i)
			}
			if rec.bottomAdjacent(r) && state.targetPoints[i][1] > r.Y1 {
				r.Y1 += 1
				reducedRectangles = append(reducedRectangles, i)
			}
		}
		state.grow(minIndex, 1)
		for _, i := range reducedRectangles {
			state.rectangles[i].O = OverlappedDirection{}
			state.grow(i, 1)
		}
	}
	return state.rectangles
}

// pointが低いやつから優先的に広げる
func solve2(state State, ratio float64) []*Rectangle {
	queue := NewPriorityQueue(nil)
	for i, r := range state.rectangles {
		queue.Push(SimpleItem{
			N: i,
			C: r.point(),
		})
	}
	for queue.IsNotEmpty() {
		item, _ := queue.Pop()
		q := item.(SimpleItem)
		if point, ok := state.grow(q.N, ratio); ok {
			queue.Push(SimpleItem{
				N: q.N,
				C: point,
			})
		}
	}
	return state.rectangles
}

// 雑にindex順にしてみる
func solve(state State) []*Rectangle {
	var queue []int
	for i := range state.rectangles {
		queue = append(queue, i)
	}
	for len(queue) > 0 {
		q := queue[0]
		queue = queue[1:]
		if _, ok := state.grow(q, 1); ok {
			queue = append(queue, q)
		}
	}
	return state.rectangles
}
func main() {
	f, _ := os.Create("debug.log")
	n := readInt()
	io.WriteString(f, fmt.Sprintf("%d\n", n))
	state := State{f: f, t: 1}
	for i := 0; i < n; i++ {
		x, y, r := readInt(), readInt(), readInt()
		io.WriteString(f, fmt.Sprintf("%d %d %d\n", x, y, r))
		state.targetPoints = append(state.targetPoints, []int{x, y})
		state.rectangles = append(state.rectangles, &Rectangle{
			X1: x,
			Y1: y,
			X2: x + 1,
			Y2: y + 1,
			R:  r,
			O:  OverlappedDirection{},
		})
	}
	state.startTime = time.Now()
	rectangles := solve6(state)
	for _, r := range rectangles {
		fmt.Printf("%d %d %d %d\n", r.X1, r.Y1, r.X2, r.Y2)
	}
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
	C float64
}

func (i SimpleItem) Priority() float64 {
	return float64(i.C)
}
func (i SimpleItem) Cost() float64 {
	return i.C
}
func (i SimpleItem) Node() int {
	return i.N
}

type Item interface {
	Priority() float64
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

// priorityの低いものをpopする
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
