package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
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
func getli(size int) []int {
	a := make([]int, size)
	list := strings.Split(read(), " ")
	for i, s := range list {
		n, _ := strconv.Atoi(s)
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
	n := geti()
	ds := getli(n)
	fmt.Println(solve(ds))
}

func solve(ds []int) int {
	if len(ds) == 1 {
		return ds[0]
	}
	var res [25]bool
	res[0] = true
	res[24] = true

	sort.Ints(ds)

	for i, d := range ds {
		if i%2 == 0 {
			if res[d] {
				return 0
			}
			res[d] = true
		} else {
			if res[24-d] {
				return 0
			}
			res[24-d] = true
		}
	}
	minnum := 100000
	counter := 1000000
	for _, b := range res {
		if b {
			minnum = min(counter, minnum)
			counter = 0
		}
		counter += 1
	}
	return minnum
}
