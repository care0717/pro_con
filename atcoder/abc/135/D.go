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
func getli(size int) ([]int) {
	a := make([]int, size)
	list := strings.Split(read(), " ")
	for i, s := range list {
		n, _ := strconv.Atoi(s)
		a[i] = n
	}
	return a
}

func get2byte(size int) ([][]byte) {
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

func matrix(n, m int) [][]int {
	mat := make([][]int, n)
	for i:=0; i<n; i++{
		mat[i] = make([]int, m)
	}
	return mat
}

func main() {
	S := strings.Split(read(), "")
	M := 1000000007
	mod := 13
	dp := matrix(len(S), mod)
	if S[0] == "?" {
		for i := 0; i< 10; i++ {
			dp[0][i] = 1
		}
	} else {
		num, _ := strconv.Atoi(S[0])
		dp[0][num] = 1
	}
	index := 1
	l := len(S)
	for _, ch := range S[1:l] {
		if ch == "?" {
			for j := 0; j< 10; j++ {
				num := j
				for k := 0; k < index; k++ {
					num = (num * 10) % 13
				}
				fmt.Println(num)
				for k, pat := range dp[index-1] {
					dp[index][(num+k)%13] += pat
				}
			}
		} else {
			num, _ :=  strconv.Atoi(ch)
			for k := 0; k < index; k++ {
				num = (num * 10) % 13
			}
			for k, pat := range dp[index-1] {
				dp[index][(num+k)%13] += pat
			}
		}
		for i := 0; i< mod; i++{
			dp[index][i] = dp[index][i]%M
		}
		index += 1
	}
	fmt.Println(dp[index-1][5])
}