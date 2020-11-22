package model

import (
	"github.com/google/go-cmp/cmp"
	"os"
	"testing"
)

func TestParseSample(t *testing.T) {
	tests := []struct {
		fileName string
		want     []Sample
	}{
		{
			fileName: "testdata/abc166_c.html",
			want: []Sample{
				{
					Input:  "4 3\n1 2 3 4\n1 3\n2 3\n2 4",
					Output: "2",
				},
				{
					Input:  "6 5\n8 6 9 1 2 1\n1 3\n4 2\n4 3\n4 6\n4 6",
					Output: "3",
				},
			},
		},
		{
			fileName: "testdata/abc166_d.html",
			want: []Sample{
				{
					Input:  "33",
					Output: "2 -1",
				},
				{
					Input:  "1",
					Output: "0 -1",
				},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.fileName, func(t *testing.T) {
			f, err := os.Open(tt.fileName)
			if err != nil {
				t.Fatal(err)
			}
			defer f.Close()
			got, err := ParseSample(f)
			if err != nil {
				t.Fatal(err)
			}
			if !cmp.Equal(got, tt.want) {
				t.Errorf("ParseSample(%s) = %v, want %v", tt.fileName, got, tt.want)
			}
		})
	}

}
