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
