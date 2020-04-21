package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
	"fmt"
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

func getli(size int) ([]int) {
	a := make([]int, size)
	list := strings.Split(read(), " ")
	for i, s := range list {
		n, _ := strconv.Atoi(s)
		a[i] = n
	}
	return a
}

func main() {
	n := geti()
	as := make([][]int, 2)
	for i := 0; i < 2; i++ {
		as[i] = getli(n)
	}
	sum := 0
	max := 0
	for i := 0; i < n; i++{
		sum = 0
		for j := 0; j < n; j++{
			if j <= i {
				sum += as[0][j]
			} else {
				sum += as[1][j]
			}
			if j == i {
				sum += as[1][j]
			}
		}
		if sum > max{
			max = sum
		}
	}
	fmt.Println(max)
}