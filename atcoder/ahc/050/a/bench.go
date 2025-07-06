package main

import (
	"fmt"
	"github.com/care0717/pro_con/atcoder/ahc/050/a/solve"
	"io/ioutil"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

type TestCase struct {
	N    int
	M    int
	Grid []string
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run bench.go a.go <input_directory>")
		os.Exit(1)
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
		solution := solve.Solve1(testCase.N, testCase.Grid)

		elapsed := time.Since(startTime)

		// スコアを計算
		rawScore := solve.CalculateScore(testCase.Grid, solution, testCase.N)
		normalizedScore := calculateNormalizedScore(rawScore, testCase.N, testCase.M)

		fmt.Printf("%-12s\t%.2f\t%8.0f\t%8.1f\n",
			file.Name(), rawScore, normalizedScore, float64(elapsed.Nanoseconds())/1e6)

		totalScore += normalizedScore
		totalTime += elapsed
		testCount++
	}

	if testCount > 0 {
		avgScore := totalScore / float64(testCount)
		avgTime := float64(totalTime.Nanoseconds()) / float64(testCount) / 1e6
		fmt.Println("----\t\t-----\t----------\t--------")
		fmt.Printf("Average\t\t\t%8.2f\t%8.1f\n", avgScore, avgTime)
		fmt.Printf("Total cases: %d\n", testCount)
	} else {
		fmt.Println("No test cases found")
	}
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
