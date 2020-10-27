package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
)

func main() {
	n := 5000
	k := 10
	max := 1000
	rand.Seed(time.Now().UnixNano())
	var ps []string
	for i := 1; i <= n; i++ {
		ps = append(ps, fmt.Sprint(i))
	}
	shuffle(ps)

	var cs []string
	for i := 1; i <= n; i++ {
		var sign int
		if rand.Intn(2) == 1 {
			sign = 1
		} else {
			sign = -1
		}
		cs = append(cs, fmt.Sprint(sign*rand.Intn(max)))
	}

	fmt.Println(n, k)
	fmt.Println(strings.Join(ps, " "))
	fmt.Println(strings.Join(cs, " "))
}
func shuffle(data []string) {
	n := len(data)
	for i := n - 1; i >= 0; i-- {
		j := rand.Intn(i + 1)
		data[i], data[j] = data[j], data[i]
	}
}
