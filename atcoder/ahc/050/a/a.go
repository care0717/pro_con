package main

import (
	"bufio"
	"fmt"
	"io"
	"io/ioutil"
	"math"
	"math/rand"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

var StartTime time.Time

func main() {
	if len(os.Args) < 2 {
		StartTime = time.Now()
		N := readInt()
		_ = readInt()

		grid := make([]string, N)
		for i := 0; i < N; i++ {
			grid[i] = ReadString()
		}
		for _, cell := range Solve3(N, grid) {
			fmt.Printf("%d %d\n", cell[0], cell[1])
		}
		return
	}
	inputDir := os.Args[len(os.Args)-1] // 最後の引数をディレクトリとして使用

	// inディレクトリ内のファイルを取得
	files, err := ioutil.ReadDir(inputDir)
	if err != nil {
		fmt.Printf("Error reading directory: %v\n", err)
		os.Exit(1)
	}

	var totalScore float64
	var testCount int
	var totalTime time.Duration

	fmt.Println("Running benchmark...")
	fmt.Println("Case\t\tScore\tNormalized\tTime(ms)")
	fmt.Println("----\t\t-----\t----------\t--------")

	for _, file := range files {
		if file.IsDir() || (!strings.HasSuffix(file.Name(), ".txt") && !strings.HasSuffix(file.Name(), ".in")) {
			continue
		}

		inputFile := filepath.Join(inputDir, file.Name())
		testCase, err := readInputFile(inputFile)
		if err != nil {
			fmt.Printf("Error reading %s: %v\n", file.Name(), err)
			continue
		}

		// 実行時間を測定
		startTime := time.Now()

		// solve関数を呼び出して解を取得
		solution := Solve3(testCase.N, testCase.Grid)

		elapsed := time.Since(startTime)

		// スコアを計算
		rawScore := CalculateScore(testCase.Grid, solution, testCase.N)
		normalizedScore := calculateNormalizedScore(rawScore, testCase.N, testCase.M)

		fmt.Printf("%-12s\t%.2f\t%8.0f\t%8.1f\n",
			file.Name(), rawScore, normalizedScore, float64(elapsed.Nanoseconds())/1e6)

		totalScore += normalizedScore
		totalTime += elapsed
		testCount++
	}

	if testCount > 0 {
		fmt.Println("----\t\t-----\t----------\t--------")
		fmt.Printf("Total\t\t\t%8.2f\n", totalScore*150/float64(testCount))
	} else {
		fmt.Println("No test cases found")
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
	rand.Seed(time.Now().UnixNano())

	emptyCells := make(map[int]int)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCells[toIndex(i, j, N)] = 1
			}
		}
	}
	initialOrder := getGreedyOrder(grid, emptyCells, N)

	// 焼きなましで最適化（1.8秒制限）
	return simulatedAnnealing(grid, initialOrder, N)
}

// 複数回貪欲解を解いてみて、一番いいやつを選ぶ
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

	return getBestGreedyOrder(grid, emptyCells, N, getGreedyOrder)
}

// 確率が10^-6以下、かつ周りに岩の少ないセルを優先するgreedyな戦略
func Solve3(N int, grid []string) [][]int {
	rand.Seed(time.Now().UnixNano())

	emptyCells := make(map[int]int)
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			if grid[i][j] == '.' {
				emptyCells[toIndex(i, j, N)] = 1
			}
		}
	}

	return getBestGreedyOrder(grid, emptyCells, N, getAdvancedGreedyOrder)
}

func getBestGreedyOrder(grid []string, emptyCells map[int]int, N int, greedy func(grid []string, emptyCells map[int]int, N int) [][]int) [][]int {
	timeLimit := 1400 * time.Millisecond
	bestOrder := greedy(grid, emptyCells, N)
	bestScore := CalculateScore(grid, bestOrder, N)
	for time.Since(StartTime) < timeLimit {
		order := greedy(grid, emptyCells, N)
		orderScore := CalculateScore(grid, order, N)
		if orderScore > bestScore {
			bestOrder = order
			bestScore = orderScore
		}
	}
	return bestOrder
}

func getGreedyOrder(grid []string, emptyCells map[int]int, N int) [][]int {
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

func getAdvancedGreedyOrder(grid []string, emptyCells map[int]int, N int) [][]int {
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

	var result [][]int
	for len(prob) > 0 {
		prob = calculateNextProb(currentGrid, prob, N)

		// 確率が10^-6以下のセルを最優先で選ぶ
		lowProbCells := make([]int, 0)
		for idx, p := range prob {
			if p <= 1e-6 {
				lowProbCells = append(lowProbCells, idx)
			}
		}

		selectedIndex := -1
		maxProb := 0.0
		for _, idx := range lowProbCells {
			i, j := fromIndex(idx, N)
			aroundP := maxProbAround(currentGrid, prob, i, j, N)
			c := rockCount(currentGrid, i, j, N) / 50
			if aroundP+c >= maxProb {
				maxProb = aroundP + c
				selectedIndex = idx
			}
		}
		if selectedIndex == -1 {
			// 確率が10^-6以下のセルがない場合は、従来通り最小確率を選ぶ
			minProb := 10000.0
			for idx, p := range prob {
				if p == 0 {
					minProb = p
					selectedIndex = idx
					break
				}
				if p < minProb {
					minProb = p
					selectedIndex = idx
				}
			}
		}

		i, j := fromIndex(selectedIndex, N)
		result = append(result, []int{i, j})
		currentGrid[i][j] = '#'
		delete(prob, selectedIndex)
	}

	return result
}

func countSurroundingRocks(grid [][]byte, i, j, N int) int {
	count := 0
	directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for _, dir := range directions {
		ni, nj := i+dir[0], j+dir[1]
		if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
			count++
		}
	}
	return count
}

// 高い確率だが、まだ3方以上閉じられてないやつを探す
func maxProbAround(grid [][]byte, prob map[int]float64, i, j, N int) float64 {
	directions := [][]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	var maxProb float64
	for _, dir := range directions {
		ni, nj := i+dir[0], j+dir[1]
		if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
			continue
		}
		p := prob[toIndex(ni, nj, N)]
		if count := countSurroundingRocks(grid, ni, nj, N); count >= 3 {
			continue
		}
		if p > maxProb {
			maxProb = p
		}
	}
	return maxProb
}

// 2個先に岩があるとプラス、斜めに岩があるとマイナスでどうか
func rockCount(grid [][]byte, i, j, N int) float64 {
	if countSurroundingRocks(grid, i, j, N) == 3 {
		return 1
	}
	directions := [][]int{{-2, 0}, {2, 0}, {0, -2}, {0, 2}}
	diagonalDirections := [][]int{{-1, -1}, {1, 1}, {1, -1}, {-1, 1}}
	count := 4
	for _, dir := range directions {
		ni, nj := i+dir[0], j+dir[1]
		if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
			count++
		}
	}
	for _, dir := range diagonalDirections {
		ni, nj := i+dir[0], j+dir[1]
		if ni < 0 || ni >= N || nj < 0 || nj >= N || grid[ni][nj] == '#' {
			count--
		}
	}

	return float64(count) / 9.0
}

func simulatedAnnealing(grid []string, order [][]int, N int) [][]int {
	timeLimit := 1800 * time.Millisecond
	K := 100

	current := make([][]int, len(order))
	copy(current, order)

	best := make([][]int, len(order))
	copy(best, order)

	currentScore := calculateScoreByUpdatePerK(K, grid, current, N)
	bestScore := currentScore

	// 焼きなましパラメータ
	startTemp := 100.0
	endTemp := 0.01

	iter := 0
	scoreCalculations := 0

	for {
		// 経過時間による温度計算
		elapsed := time.Since(StartTime)
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

		neighborScore := calculateScoreByUpdatePerK(K, grid, neighbor, N)
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
		if time.Since(StartTime) > timeLimit {
			break
		}
	}
	//fmt.Println(iter)
	return best
}

func toIndex(i, j, N int) int {
	return i*N + j
}

func fromIndex(idx, N int) (int, int) {
	return idx / N, idx % N
}
func CalculateScore(grid []string, order [][]int, N int) float64 {
	return calculateScoreByUpdatePerK(1, grid, order, N)
}

func calculateScoreByUpdatePerK(K int, grid []string, order [][]int, N int) float64 {
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
	for i, pos := range order {
		bi, bj := pos[0], pos[1]
		// prob更新をサボる
		if i%K == 0 {
			prob = calculateNextProb(currentGrid, prob, N)
		}
		// 岩を置く前の生存確率を計算
		targetIdx := toIndex(bi, bj, N)
		if hitProb, exists := prob[targetIdx]; exists {
			life -= hitProb
		}
		// スコアに追加
		totalScore += life
		// 岩を置く
		currentGrid[bi][bj] = '#'
		delete(prob, targetIdx)
	}

	return totalScore
}

func calculateNextProb(grid [][]byte, prob map[int]float64, N int) map[int]float64 {
	nextProb := make(map[int]float64)
	// 各位置からロボットが4方向に移動
	for idx, probability := range prob {
		if _, ok := nextProb[idx]; !ok {
			nextProb[idx] = 0
		}
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

type TestCase struct {
	N    int
	M    int
	Grid []string
}

func readInputFile(filename string) (*TestCase, error) {
	content, err := ioutil.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(content), "\n")
	if len(lines) < 1 {
		return nil, fmt.Errorf("invalid input format")
	}

	// 最後の空行を除去
	for len(lines) > 0 && strings.TrimSpace(lines[len(lines)-1]) == "" {
		lines = lines[:len(lines)-1]
	}

	// 最初の行からNとMを読み取り
	firstLine := strings.Fields(lines[0])
	if len(firstLine) != 2 {
		return nil, fmt.Errorf("invalid first line format")
	}

	N, err := strconv.Atoi(firstLine[0])
	if err != nil {
		return nil, fmt.Errorf("invalid N: %v", err)
	}

	M, err := strconv.Atoi(firstLine[1])
	if err != nil {
		return nil, fmt.Errorf("invalid M: %v", err)
	}

	if len(lines) < N+1 {
		return nil, fmt.Errorf("insufficient grid lines")
	}

	grid := make([]string, N)
	for i := 0; i < N; i++ {
		grid[i] = lines[i+1]
		if len(grid[i]) != N {
			return nil, fmt.Errorf("invalid grid line %d: expected length %d, got %d", i, N, len(grid[i]))
		}
	}

	return &TestCase{
		N:    N,
		M:    M,
		Grid: grid,
	}, nil
}

func calculateNormalizedScore(rawScore float64, N, M int) float64 {
	// 問題文に従った正規化スコア計算
	// score = round(10^6 * E / (N^2 - M - 1))
	ub := float64(N*N - M - 1)
	return math.Round(rawScore * 1e6 / ub)
}
