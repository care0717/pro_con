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

func Geti() int {
	n, _ := strconv.Atoi(read())
	return n
}

func Getli(size int) ([]int) {
	a := make([]int, size)
	list := strings.Split(read(), " ")
	for i, s := range list {
		n, _ := strconv.Atoi(s)
		a[i] = n
	}
	return a
}

func main() {
	n := Getli(2)
	fmt.Println(n[0]-n[1]+1)
}