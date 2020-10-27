package e_test

import (
	"github.com/care0717/pro_con/atcoder/abc/173/e"
	"testing"
)

func TestSolve(t *testing.T) {
	tests := []struct {
		n    int
		k    int
		list []int
		want int
	}{
		{n: 4, k: 2, list: []int{1, 2, -3, -4}, want: 12},
		{4, 3, []int{-1, -2, -3, -4}, -6},
		{4, 2, []int{-1, -2, -3, -4}, 12},
		{2, 1, []int{-1, 1000000000}, 1000000000},
		{10, 10, []int{1000000000, 100000000, 10000000, 1000000, 100000, 10000, 1000, 100, 10, 1}, 999983200},
		{3, 2, []int{10, 0, -3}, 0},
		{4, 3, []int{10, 0, 0, 3}, 0},
		{4, 3, []int{-10, 0, 0, -3}, 0},
		{3, 2, []int{10, -1, -3}, 3},
		{4, 3, []int{0, -1, -3, 5}, 15},
		{4, 3, []int{0, -1, -3, -5}, 0},
		{4, 2, []int{0, -1, -3, -5}, 15},
		{6, 4, []int{0, -1, -3, 2, -3, -5}, 45},
		{5, 4, []int{0, 2, 2, 3, 3}, 36},
		{5, 4, []int{1, 2, 2, 3, -4}, 12},
		{5, 2, []int{1, 2, -2, -3, 4}, 8},
		{4, 2, []int{3, -3, -2, 1}, 6},
		{4, 2, []int{3, -3, 2, -1}, 6},
	}
	for _, tt := range tests {
		if got := e.Solve(tt.n, tt.k, tt.list); got != tt.want {
			t.Errorf("solve(%v, %v, %v) = %v, want %v", tt.n, tt.k, tt.list, got, tt.want)
		}
	}
}
