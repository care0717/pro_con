package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"math"
	"math/rand"
	"os"
	"sort"
	"strconv"
	"time"
)

func solve(N int, grid []string) [][]int {
	startTime := time.Now()
	rand.Seed(time.Now().UnixNano())

	var emptyCells [][]int
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCells = append(emptyCells, []int{i, j})
			}
		}
	}
	// 初期解をヒューリスティックで生成
	initialOrder := getInitialOrder(grid, emptyCells, N)

	// 焼きなましで最適化（1.8秒制限）
	return simulatedAnnealing(startTime, grid, initialOrder, N)
}

func main() {
	N := readInt()
	_ = readInt()

	grid := make([]string, N)
	for i := 0; i < N; i++ {
		grid[i] = ReadString()
	}
	for _, cell := range solve(N, grid) {
		fmt.Printf("%d %d\n", cell[0], cell[1])
	}
}

func getInitialOrder(grid []string, emptyCells [][]int, N int) [][]int {
	// 動的優先度キューベースの追い込み漁戦略
	return getGreedyInitialOrder(grid, emptyCells, N)
}

func getGreedyInitialOrder(grid []string, emptyCells [][]int, N int) [][]int {
	type CellWithProb struct {
		pos  []int
		prob float64
	}

	// 各空きセルに到達する確率を計算
	var cellProbs []CellWithProb

	// 現在のグリッド状態をコピー
	currentGrid := make([][]byte, N)
	for i := 0; i < N; i++ {
		currentGrid[i] = []byte(grid[i])
	}
	emptyCount := len(emptyCells)
	prob := make(map[int]float64, N)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if currentGrid[i][j] == '.' {
				prob[toIndex(i, j, N)] = 1.0 / float64(emptyCount)
			}
		}
	}
	// 各空きセルに対してロボットが到達する確率を簡易計算
	nextProb := calculateNextProb(currentGrid, prob, N)
	for _, cell := range emptyCells {
		cellProbs = append(cellProbs, CellWithProb{[]int{cell[0], cell[1]}, nextProb[toIndex(cell[0], cell[1], N)]})
	}

	// 確率の低い順（先に埋める順）にソート
	sort.Slice(cellProbs, func(i, j int) bool {
		return cellProbs[i].prob < cellProbs[j].prob
	})

	var result [][]int
	for _, p := range cellProbs {
		result = append(result, p.pos)
	}

	return result
}

func calculateReachabilityScore(grid [][]byte, targetRow, targetCol, N int) float64 {
	// 各位置からターゲットに到達できる確率を計算
	score := 0.0
	emptyCount := 0

	// 全ての空きマスから計算
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCount++

				// この位置から4方向に移動してターゲットに到達できるかチェック
				directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
				reachCount := 0

				for _, dir := range directions {
					// この方向に移動してどこに到達するか
					i2, j2 := i, j
					for {
						ni, nj := i2+dir[0], j2+dir[1]
						if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
							break
						}
						i2, j2 = ni, nj
					}

					// ターゲット位置に到達した場合
					if i2 == targetRow && j2 == targetCol {
						reachCount++
					}
				}

				// この位置からターゲットに到達する確率を加算
				score += float64(reachCount) / 4.0
			}
		}
	}

	if emptyCount == 0 {
		return 0.0
	}

	// 全体の確率を正規化
	return score / float64(emptyCount)
}

func simulatedAnnealing(startTime time.Time, grid []string, order [][]int, N int) [][]int {
	timeLimit := 1800 * time.Millisecond // 1.8秒

	current := make([][]int, len(order))
	copy(current, order)

	best := make([][]int, len(order))
	copy(best, order)

	currentScore := calculateScore(grid, current, N)
	bestScore := currentScore

	// 焼きなましパラメータ
	startTemp := 100.0
	endTemp := 0.01

	iter := 0
	scoreCalculations := 0

	for time.Since(startTime) < timeLimit {
		// 経過時間による温度計算
		elapsed := time.Since(startTime)
		progress := float64(elapsed) / float64(timeLimit)
		temp := startTemp * math.Pow(endTemp/startTemp, progress)

		// 近傍解生成（複数の戦略）
		neighbor := make([][]int, len(current))
		copy(neighbor, current)

		// 近い位置のスワップを優先（局所的な変更）
		r := rand.Float64()
		if r < 0.7 {
			// 近い位置でのスワップ
			i := rand.Intn(len(neighbor))
			maxDist := min(10, len(neighbor)/4)
			j := i + rand.Intn(maxDist*2+1) - maxDist
			if j < 0 {
				j = 0
			}
			if j >= len(neighbor) {
				j = len(neighbor) - 1
			}
			neighbor[i], neighbor[j] = neighbor[j], neighbor[i]
		} else if r < 0.9 {
			// 完全ランダムスワップ
			i, j := rand.Intn(len(neighbor)), rand.Intn(len(neighbor))
			neighbor[i], neighbor[j] = neighbor[j], neighbor[i]
		} else {
			// 区間反転
			if len(neighbor) > 1 {
				i, j := rand.Intn(len(neighbor)), rand.Intn(len(neighbor))
				if i > j {
					i, j = j, i
				}
				// [i, j]区間を反転
				for k := 0; k < (j-i+1)/2; k++ {
					neighbor[i+k], neighbor[j-k] = neighbor[j-k], neighbor[i+k]
				}
			}
		}

		neighborScore := calculateScore(grid, neighbor, N)
		scoreCalculations++

		// 受諾判定
		if neighborScore > currentScore ||
			rand.Float64() < math.Exp((neighborScore-currentScore)/temp) {
			current = neighbor
			currentScore = neighborScore

			if currentScore > bestScore {
				best = make([][]int, len(current))
				copy(best, current)
				bestScore = currentScore
			}
		}

		iter++
	}

	return best
}

func toIndex(i, j, N int) int {
	return i*N + j
}

func fromIndex(idx, N int) (int, int) {
	return idx / N, idx % N
}
func calculateScore(grid []string, order [][]int, N int) float64 {

	// 現在のグリッド状態をコピー
	currentGrid := make([][]byte, N)
	for i := 0; i < N; i++ {
		currentGrid[i] = []byte(grid[i])
	}

	// 初期確率分布：全ての空きマスに均等に分布
	emptyCount := 0
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if currentGrid[i][j] == '.' {
				emptyCount++
			}
		}
	}

	if emptyCount == 0 {
		return 0.0
	}

	// 確率分布を初期化
	prob := make(map[int]float64, N)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if currentGrid[i][j] == '.' {
				prob[toIndex(i, j, N)] = 1.0 / float64(emptyCount)
			}
		}
	}

	totalScore := 0.0
	life := 1.0

	// 各ステップをシミュレーション
	for _, pos := range order {
		bi, bj := pos[0], pos[1]

		nextProb := calculateNextProb(currentGrid, prob, N)

		// 岩を置く前の生存確率を計算
		targetIdx := toIndex(bi, bj, N)
		if hitProb, exists := nextProb[targetIdx]; exists {
			life -= hitProb
		}
		// 岩を置く
		currentGrid[bi][bj] = '#'
		delete(nextProb, targetIdx)

		// スコアに追加
		totalScore += life

		// 確率分布を更新
		prob = nextProb
	}

	return totalScore
}

func calculateNextProb(grid [][]byte, prob map[int]float64, N int) map[int]float64 {
	nextProb := make(map[int]float64)
	// 各位置からロボットが4方向に移動
	for idx, probability := range prob {
		i, j := fromIndex(idx, N)
		// 4方向への移動をシミュレーション
		directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

		for _, dir := range directions {
			// この方向に岩にぶつかるまで移動
			i2, j2 := i, j
			for {
				ni, nj := i2+dir[0], j2+dir[1]
				if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
					break
				}
				i2, j2 = ni, nj
			}
			// 移動後の位置に確率を追加（1/4の確率で各方向）
			nextProb[toIndex(i2, j2, N)] += probability * 0.25
		}
	}
	return nextProb
}

func estimateSurvivalProbability(grid [][]byte, rockRow, rockCol, N int) float64 {
	// 簡易的な生存確率推定
	// 岩を置いた場合に、ロボットがその位置に到達する確率の逆数

	// 周辺の空きスペースの数を数える
	emptySpaces := 0
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptySpaces++
			}
		}
	}

	if emptySpaces == 0 {
		return 0.0
	}

	// 単純な近似：周辺に岩が多いほど生存確率が高い
	nearbyRocks := 0
	directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	for _, dir := range directions {
		nr, nc := rockRow+dir[0], rockCol+dir[1]
		if nr >= 0 && nr < N && nc >= 0 && nc < N && grid[nr][nc] == '#' {
			nearbyRocks++
		}
	}

	// 基本生存確率 + 周辺岩ボーナス
	return 0.8 + float64(nearbyRocks)*0.05
}

func countNearbyRocks(grid []string, row, col, N int) int {
	count := 0
	directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

	for _, dir := range directions {
		nr, nc := row+dir[0], col+dir[1]
		if nr >= 0 && nr < N && nc >= 0 && nc < N && grid[nr][nc] == '#' {
			count++
		}
	}
	return count
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
