package main

import (
	"bufio"
	"fmt"
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

func transpose(a [][]byte) [][]byte {
	n := len(a)
	w := len(a[0])

	var res [][]byte
	for j := 0; j < w; j++ {
		temp := make([]byte, n)
		for i := 0; i < n; i++ {
			temp[i] = a[i][j]
		}
		res = append(res, temp)
	}
	return res
}

func uniq(a []byte) []byte {
	m := make(map[byte]bool)
	var uniq []byte

	for _, ele := range a {
		if !m[ele] {
			m[ele] = true
			uniq = append(uniq, ele)
		}
	}

	return uniq
}

func main() {
	hs := getli(2)
	H := hs[0]
	W := hs[1]
	a := get2byte(H)
	var temp, res [][]byte
	for i := 0; i < H; i++ {
		fmt.Println(!(len(uniq(a[i])) == 1 && uniq(a[i])[0] == '.'))
		if !(len(uniq(a[i])) == 1 && uniq(a[i])[0] == '.') {
			temp = append(temp, a[i])
		}
	}
	h := len(temp)
	temp = transpose(temp)
	fmt.Println(temp)
	fmt.Println(W)

	for i := 0; i < W; i++ {
		if !(len(uniq(temp[i])) == 1 && uniq(temp[i])[0] == '.') {
			res = append(res, temp[i])
		}
	}
	fmt.Println('a')

	w := len(res)
	res = transpose(res)
	for i := 0; i < h; i++ {
		for j := 0; j < w; j++ {
			fmt.Print(string(res[i][j]))
		}
		fmt.Println()
	}
}
