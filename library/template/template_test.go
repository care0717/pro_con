package main

import (
	"github.com/google/go-cmp/cmp"
	"testing"
)

func TestPrimeFactorize(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want []struct {
			Prime int
			Index int
		}
	}{
		{
			name: "normal",
			n:    100,
			want: []struct {
				Prime int
				Index int
			}{{2, 2}, {5, 2}},
		},
		{
			name: "big num",
			n:    72149909,
			want: []struct {
				Prime int
				Index int
			}{{13, 1}, {5549993, 1}},
		},
		{
			name: "corner",
			n:    2,
			want: []struct {
				Prime int
				Index int
			}{{2, 1}},
		},
		{
			name: "invalid num",
			n:    1,
			want: nil,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := primeFactorize(tt.n); !cmp.Equal(got, tt.want) {
				t.Errorf("primeFactorize(%d) = %v, want %v", tt.n, got, tt.want)
			}
		})
	}
}

func TestDivisor(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want []int
	}{
		{
			name: "normal",
			n:    12,
			want: []int{1, 2, 3, 4, 6, 12},
		},
		{
			name: "0",
			n:    0,
			want: []int{},
		},
		{
			name: "1",
			n:    1,
			want: []int{1},
		},
		{
			name: "prime",
			n:    7,
			want: []int{1, 7},
		},
		{
			name: "square",
			n:    121,
			want: []int{1, 11, 121},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := divisor(tt.n); !cmp.Equal(got, tt.want) {
				t.Errorf("primeFactorize(%d) = %v, want %v", tt.n, got, tt.want)
			}
		})
	}
}

func TestUnionFind(t *testing.T) {
	type pair struct {
		x, y int
	}
	tests := []struct {
		name          string
		size          int
		unites        []pair
		expectSame    []pair
		expectNotSame []pair
	}{
		{
			name: "normal",
			size: 3,
			unites: []pair{
				{x: 0, y: 1},
			},
			expectSame: []pair{
				{x: 1, y: 0},
			},
			expectNotSame: []pair{
				{x: 0, y: 2},
				{x: 1, y: 2},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			u := NewUnionFind(tt.size)
			for _, p := range tt.unites {
				u.Unite(p.x, p.y)
			}
			for _, p := range tt.expectSame {
				if got := u.Same(p.x, p.y); !got {
					t.Errorf("u.Same(%d, %d) expect true but got false", p.x, p.y)
				}
			}
			for _, p := range tt.expectNotSame {
				if got := u.Same(p.x, p.y); got {
					t.Errorf("u.Same(%d, %d) expect false but got true", p.x, p.y)
				}
			}
		})
	}
}

func TestPriorityQueue(t *testing.T) {

	tests := []struct {
		name      string
		items     []Item
		pushItems []Item
		expected  []Item
	}{
		{
			name: "valid NewPQ",
			items: []Item{
				SimpleItem{
					C: 2,
				},
				SimpleItem{
					C: 1,
				},
				SimpleItem{
					C: 3,
				},
			},
			pushItems: nil,
			expected: []Item{
				SimpleItem{
					C: 1,
				},
				SimpleItem{
					C: 2,
				},
				SimpleItem{
					C: 3,
				},
			},
		},
		{
			name:  "valid Push",
			items: nil,
			pushItems: []Item{
				SimpleItem{
					C: 5,
				},
				SimpleItem{
					C: -3,
				},
				SimpleItem{
					C: 2,
				},
			},
			expected: []Item{
				SimpleItem{
					C: -3,
				},
				SimpleItem{
					C: 2,
				},
				SimpleItem{
					C: 5,
				},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			pq := NewPriorityQueue(tt.items)
			for _, i := range tt.pushItems {
				pq.Push(i)
			}
			for _, e := range tt.expected {
				got, _ := pq.Pop()
				if !cmp.Equal(got, e) {
					t.Errorf("pq.Pop = %v, want %v", got, e)
				}
			}
			if pq.IsNotEmpty() {
				t.Errorf("pq is not empty")
			}
		})
	}
}
