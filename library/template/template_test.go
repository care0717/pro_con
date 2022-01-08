package main

import (
	"github.com/google/go-cmp/cmp"
	"math/rand"
	"testing"
)

func TestCalcGcd(t *testing.T) {
	t.Parallel()
	tests := []struct {
		name string
		as   []int
		want int
	}{
		{
			name: "normal",
			as:   []int{18, 24, 33},
			want: 3,
		},
		{
			name: "single",
			as:   []int{13},
			want: 13,
		},
		{
			name: "max1",
			as:   []int{11, 22},
			want: 11,
		},
		{
			name: "max2",
			as:   []int{42, 84},
			want: 42,
		},
		{
			name: "prime",
			as:   []int{13, 53},
			want: 1,
		},
		{
			name: "minus, plus",
			as:   []int{-10, 15},
			want: 5,
		},
		{
			name: "plus, minus",
			as:   []int{14, -7},
			want: 7,
		},
		{
			name: "minus, minus",
			as:   []int{-4, -6},
			want: 2,
		},
		{
			name: "zero",
			as:   []int{0, 2},
			want: 2,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			if got := calcGcd(tt.as...); !cmp.Equal(got, tt.want) {
				t.Errorf("calcGcd(%d) = %v, want %v", tt.as, got, tt.want)
			}
		})
	}
}

func TestPrimeFactorize(t *testing.T) {
	t.Parallel()
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
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			if got := primeFactorize(tt.n); !cmp.Equal(got, tt.want) {
				t.Errorf("primeFactorize(%d) = %v, want %v", tt.n, got, tt.want)
			}
		})
	}
}

func TestDivisor(t *testing.T) {
	t.Parallel()
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
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			if got := divisor(tt.n); !cmp.Equal(got, tt.want) {
				t.Errorf("primeFactorize(%d) = %v, want %v", tt.n, got, tt.want)
			}
		})
	}
}

func TestUnionFind(t *testing.T) {
	t.Parallel()
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
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
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
	t.Parallel()
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
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
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

func TestAvlTree_EmptyTree(t *testing.T) {
	t.Parallel()
	at := NewAvlTree()

	shouldFalse := at.Search(10)
	if shouldFalse {
		t.Errorf("at.Search = %v, want false", shouldFalse)
	}

	_, shouldFalse = at.LowerBound(10)
	if shouldFalse {
		t.Errorf("at.LowerBound = %v, want false", shouldFalse)
	}

	_, shouldFalse = at.UpperBound(10)
	if shouldFalse {
		t.Errorf("at.UpperBound = %v, want false", shouldFalse)
	}

	_, shouldFalse = <-at.Iter()
	if shouldFalse {
		t.Errorf("at.Iter = %v, want false", shouldFalse)
	}
}

func TestAvlTree_Search(t *testing.T) {
	t.Parallel()
	at := NewAvlTree()
	at.Insert(4)
	tests := []struct {
		name   string
		input  int
		expect bool
	}{
		{
			name:   "is exists",
			input:  4,
			expect: true,
		},
		{
			name:   "not exists",
			input:  100,
			expect: false,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			got := at.Search(tt.input)
			if got != tt.expect {
				t.Errorf("at.Search = %v, want %v", got, true)
			}
		})
	}
}

func TestAvlTree_LowerBound(t *testing.T) {
	t.Parallel()
	at := NewAvlTree()
	at.Insert(5)
	at.Insert(10)
	at.Insert(15)
	tests := []struct {
		name   string
		input  int
		ok     bool
		expect int
	}{
		{
			name:  "no lower",
			input: -1,
			ok:    false,
		},
		{
			name:   "equal elm",
			input:  5,
			ok:     true,
			expect: 5,
		},
		{
			name:   "has lower1",
			input:  7,
			ok:     true,
			expect: 5,
		},
		{
			name:   "has lower2",
			input:  11,
			ok:     true,
			expect: 10,
		},
		{
			name:   "over max elm",
			input:  100,
			ok:     true,
			expect: 15,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			got, ok := at.LowerBound(tt.input)
			if ok != tt.ok {
				t.Errorf("at.LowerBound ok = %v, want %v", ok, tt.ok)
			}
			if !ok {
				return
			}
			if got != tt.expect {
				t.Errorf("at.LowerBound = %v, want %v", got, tt.expect)
			}
		})
	}
}

func TestAvlTree_UpperBound(t *testing.T) {
	t.Parallel()
	at := NewAvlTree()
	at.Insert(5)
	at.Insert(10)
	at.Insert(15)
	tests := []struct {
		name   string
		input  int
		ok     bool
		expect int
	}{
		{
			name:   "under min elm",
			input:  -1,
			ok:     true,
			expect: 5,
		},
		{
			name:   "equal elm",
			input:  5,
			ok:     true,
			expect: 5,
		},
		{
			name:   "has upper1",
			input:  7,
			ok:     true,
			expect: 10,
		},
		{
			name:   "has upper2",
			input:  11,
			ok:     true,
			expect: 15,
		},
		{
			name:  "no upper",
			input: 100,
			ok:    false,
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			got, ok := at.UpperBound(tt.input)
			if ok != tt.ok {
				t.Errorf("at.UpperBound ok = %v, want %v", ok, tt.ok)
			}
			if !ok {
				return
			}
			if got != tt.expect {
				t.Errorf("at.UpperBound = %v, want %v", got, tt.expect)
			}
		})
	}
}

func TestAvlTree_Iter(t *testing.T) {
	t.Parallel()
	inputLen := 100
	inputs := make([]int, inputLen)
	for i := 0; i < inputLen; i++ {
		inputs[i] = i
	}
	rand.Shuffle(len(inputs), func(i, j int) { inputs[i], inputs[j] = inputs[j], inputs[i] })

	at := NewAvlTree()
	for _, i := range inputs {
		at.Insert(i)
	}
	c := at.Iter()
	var expect int
	for got := range c {
		if got != expect {
			t.Errorf("at.Iter() = %v, want %v", got, expect)
		}
		expect++
	}
}
