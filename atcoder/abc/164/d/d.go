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
	sc.Buffer([]byte{}, math.MaxInt64)
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

func main() {
	N := 2019
	s := read()
	m := make([]int, N)
	total := 0
	ten := 1
	for i := 0; i < len(s); i++ {
		n, _ := strconv.Atoi(s[len(s)-1-i : len(s)-i])
		total += (n * ten) % N
		ten = (ten * 10) % N
		m[total%N]++
	}
	sum := m[0]
	for _, v := range m {
		sum += v * (v - 1) / 2
	}
	fmt.Println(sum)
}
