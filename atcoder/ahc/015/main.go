package main

import (
	"bufio"
	"io"
	"os"
	"strconv"
	"time"
)

func main() {
	fs := readIntSlice(100)
}

var (
	// ReadString returns a WORD string.
	ReadString func() string
	MOD        = 1000000007
	INF        = 9223372036854775807
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

func readFloat64() float64 {
	f, _ := strconv.ParseFloat(ReadString(), 10)
	return f
}

// 10 11 12 => [10, 11, 12]
func readIntSlice(size int) []int {
	a := make([]int, size)
	for i := 0; i < size; i++ {
		a[i] = readInt()
	}
	return a
}

type TimeKeeper struct {
	start_time     time.Time
	time_threshold int // milliseconds
	end_turn       int
}

func NewTimeKeeper(time_threshold, end_turn int) *TimeKeeper {
	return &TimeKeeper{
		start_time:     time.Now(),
		time_threshold: time_threshold,
		end_turn:       end_turn,
	}
}

func (t *TimeKeeper) isTimeOver() bool {

}
