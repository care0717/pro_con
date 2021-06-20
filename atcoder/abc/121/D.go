package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

var sc = bufio.NewScanner(os.Stdin)

func read() string {
	sc.Scan()
	return sc.Text()
}

func geti() int {
	n, _ := strconv.Atoi(read())
	return n
}

// 10 11 12 => [10, 11, 12]
func getli(size int) []int64 {
	a := make([]int64, size)
	list := strings.Split(read(), " ")
	for i, s := range list {
		n, _ := strconv.ParseInt(s, 10, 64)
		a[i] = n
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

func min(nums ...int) int {
	if len(nums) == 0 {
		panic("funciton min() requires at least one argument.")
	}
	res := nums[0]
	for i := 0; i < len(nums); i++ {
		res = int(math.Min(float64(res), float64(nums[i])))
	}
	return res
}

func main() {
	ab := getli(2)
	a := ab[0]
	b := ab[1]
	fmt.Println(solve(a, b))
}

func solve(a, b int64) int64 {
	if a%2 == 0 && b%2 == 0 {
		return b ^ (((b - a) / 2) % 2)
	} else if a%2 == 0 && b%2 == 1 {
		return ((b - a + 1) / 2) % 2
	} else if a%2 == 1 && b%2 == 0 {
		return a ^ b ^ (((b - a - 1) / 2) % 2)
	} else {
		return a ^ (((b - a) / 2) % 2)
	}

}
