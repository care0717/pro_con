package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
)

var (
	// ReadString returns a WORD string.
	ReadString func() string
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

// 10 11 12 => [10, 11, 12]
func readIntSlice(size int) []int {
	a := make([]int, size)
	for i := 0; i < size; i++ {
		a[i] = readInt()
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

func main() {
	n, m, k := readInt(), readInt(), readInt()
	as := readIntSlice(n)
	bs := readIntSlice(m)
	var maxASum int
	var maxAIndex = -1
	var readACount int
	for i, a := range as {
		if maxASum+a <= k {
			maxASum += a
			maxAIndex = i
			readACount++
		} else {
			break
		}
	}
	rest := k - maxASum
	var maxBSum int
	var maxBIndex = -1
	var readBCount int
	for i, b := range bs {
		if maxBSum+b <= rest {
			maxBSum += b
			maxBIndex = i
			readBCount++
		} else {
			break
		}
	}
	rest = rest - maxBSum
	cur := readACount + readBCount
	result := cur
	aIndex := maxAIndex
	bIndex := maxBIndex
	for aIndex >= 0 && bIndex < m-1 {
		rest += as[aIndex]
		aIndex--
		cur--
		for bIndex < m-1 && rest >= bs[bIndex+1] {
			rest -= bs[bIndex+1]
			cur++
			bIndex++
		}
		if result < cur {
			result = cur
		}
	}
	fmt.Println(result)
}
