package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"math/rand"
	"os"
	"strconv"
	"time"
)

func main() {
	N := readInt()
	_ = readInt()

	grid := make([]string, N)
	for i := 0; i < N; i++ {
		grid[i] = ReadString()
	}
	for _, cell := range Solve2(N, grid) {
		fmt.Printf("%d %d\n", cell[0], cell[1])
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

func Solve2(N int, grid []string) [][]int {
	startTime := time.Now()
	rand.Seed(time.Now().UnixNano())

	emptyCells := make(map[int]int)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCells[toIndex(i, j, N)] = 1
			}
		}
	}
	// 初期解をヒューリスティックで生成
	initialOrder := getInitialOrder(grid, emptyCells, N)

	// 焼きなましで最適化（1.8秒制限）
	return simulatedAnnealing(startTime, grid, initialOrder, N)
	//return initialOrder
}

func Solve1(N int, grid []string) [][]int {
	rand.Seed(time.Now().UnixNano())

	emptyCells := make(map[int]int)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCells[toIndex(i, j, N)] = 1
			}
		}
	}
	// 初期解をヒューリスティックで生成
	initialOrder := getInitialOrder(grid, emptyCells, N)

	// 焼きなましで最適化（1.8秒制限）
	//return simulatedAnnealing(startTime, grid, initialOrder, N)
	return initialOrder
}

func getInitialOrder(grid []string, emptyCells map[int]int, N int) [][]int {
	// 動的優先度キューベースの追い込み漁戦略
	return getGreedyInitialOrder(grid, emptyCells, N)
}

func getGreedyInitialOrder(grid []string, emptyCells map[int]int, N int) [][]int {
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
	var result [][]int
	for len(prob) > 0 {
		nextProb := calculateNextProb(currentGrid, prob, N)
		minProb := 10000.0
		var minIndex int
		for idx, p := range nextProb {
			if p == 0 {
				minProb = p
				minIndex = idx
				break
			}
			if p < minProb {
				minProb = p
				minIndex = idx
			}
		}
		i, j := fromIndex(minIndex, N)
		result = append(result, []int{i, j})
		currentGrid[i][j] = '#'
		delete(nextProb, minIndex)
		prob = nextProb
	}

	return result
}

func simulatedAnnealing(startTime time.Time, grid []string, order [][]int, N int) [][]int {
	timeLimit := 1800 * time.Millisecond // 1.8秒

	current := make([][]int, len(order))
	copy(current, order)

	best := make([][]int, len(order))
	copy(best, order)

	currentScore := CalculateScore(grid, current, N)
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

		neighborScore := CalculateScore(grid, neighbor, N)
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
	fmt.Println(iter)
	return best
}

func toIndex(i, j, N int) int {
	return i*N + j
}

func fromIndex(idx, N int) (int, int) {
	return idx / N, idx % N
}
func CalculateScore(grid []string, order [][]int, N int) float64 {

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
