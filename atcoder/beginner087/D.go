package main

import "fmt"
import . "../template"

func main() {
	as := Getli(2)
	n := as[0]
	m := as[1]
	ls := make([][]int, m)
	for i:=0; i<m; i++ {
		ls[i] = Getli(3)
	}
	var dist [][][]
	fmt.Println(dist)

	for i:=0; i<m; i++ {
		dist[ls[i][0]-1][ls[i][1]-1] = ls[i][2]
		dist[ls[i][1]-1][ls[i][0]-1] = -ls[i][2]
	}
	fmt.Println(dist)
}