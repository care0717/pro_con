package main

import (
	"bufio"
	"container/heap"
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

const (
	MM = 5000
	MT = 260000
)

func main() {
	N, M, S := readInt(), readInt(), readInt()
	var U, V, A, B, C, D []int
	var G [MT + 5][]Edge
	for i := 0; i < M; i++ {
		u, v, a, b := readInt(), readInt(), readInt(), readInt()
		u--
		v--
		U = append(U, u)
		V = append(V, v)
		A = append(A, a)
		B = append(B, b)
	}
	for i := 0; i < N; i++ {
		c, d := readInt(), readInt()
		C = append(C, c)
		D = append(D, d)
	}

	for coin := 0; coin <= MM; coin++ {
		for i := 0; i < M; i++ {
			u, v, a, b := U[i], V[i], A[i], B[i]
			if coin-a >= 0 {
				G[u+N*coin] = append(G[u+N*coin], Edge{
					to:   v + (coin-a)*N,
					cost: b,
				})
				G[v+N*coin] = append(G[v+N*coin], Edge{
					to:   u + (coin-a)*N,
					cost: b,
				})
			}
		}

		for i := 0; i < N; i++ {
			c, d := C[i], D[i]
			tocoin := min(coin+c, MM)
			if coin == tocoin {
				continue
			}

			G[i+N*coin] = append(G[i+N*coin], Edge{
				to:   i + N*tocoin,
				cost: d,
			})
		}
	}
	dp, _ := dijkstra(min(S, MM)*N, MT, G[:MT])
	for i := 1; i < N; i++ {
		ans := INF_DIJK
		for j := 0; j <= MM; j++ {
			if ans > dp[i+N*j] {
				ans = dp[i+N*j]
			}
		}
		fmt.Println(ans)
	}
}

type (
	Edge struct {
		to   int
		cost int
	}
	Vertex struct {
		pri int
		id  int
	}
)

type VisitStatus int

const (
	INF_DIJK = 1 << 60
	// for dijkstra, prim, and so on
	WHITE VisitStatus = iota + 1
	GRAY
	BLACK
)

type VertexPQ []*Vertex

func (pq VertexPQ) Len() int           { return len(pq) }
func (pq VertexPQ) Less(i, j int) bool { return pq[i].pri < pq[j].pri } // <: ASC, >: DESC
func (pq VertexPQ) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}
func (pq *VertexPQ) Push(x interface{}) {
	item := x.(*Vertex)
	*pq = append(*pq, item)
}
func (pq *VertexPQ) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func dijkstra(sid, n int, AG [][]Edge) ([]int, []int) {
	dp := make([]int, n)
	colors, parents := make([]VisitStatus, n), make([]int, n)
	for i := 0; i < n; i++ {
		dp[i] = INF_DIJK
		colors[i], parents[i] = WHITE, -1
	}

	temp := make(VertexPQ, 0, 100000+5)
	pq := &temp
	heap.Init(pq)
	heap.Push(pq, &Vertex{pri: 0, id: sid})
	dp[sid] = 0
	colors[sid] = GRAY

	for pq.Len() > 0 {
		pop := heap.Pop(pq).(*Vertex)

		colors[pop.id] = BLACK

		if pop.pri > dp[pop.id] {
			continue
		}

		for _, e := range AG[pop.id] {
			if colors[e.to] == BLACK {
				continue
			}

			if dp[e.to] > dp[pop.id]+e.cost {
				dp[e.to] = dp[pop.id] + e.cost
				heap.Push(pq, &Vertex{pri: dp[e.to], id: e.to})
				colors[e.to], parents[e.to] = GRAY, pop.id
			}
		}
	}

	return dp, parents
}
