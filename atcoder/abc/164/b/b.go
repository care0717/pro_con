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

func main() {
	tmp := getli(4)
	a, b, c, d := tmp[0], tmp[1], tmp[2], tmp[3]
	n := math.Ceil(float64(a) / float64(d))
	m := math.Ceil(float64(c) / float64(b))

	if n >= m {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
