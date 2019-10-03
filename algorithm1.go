package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

var closingBrackets []byte
var store []byte

func main() {
	count, err := strconv.Atoi(os.Args[1])
	if err == nil {
		fmt.Println(count)
	}
	s := time.Now()
	variants := brackets(count)
	t := time.Now().Sub(s)

	fmt.Println("time", t)
	fmt.Println("variants", len(variants))
}

func brackets(n int) []string {
	if n < 1 {
		return nil
	}
	closingBrackets = nil
	for i := 0; i < n; i++ {
		closingBrackets = append(closingBrackets, ')')
	}
	//debug.SetGCPercent(-1)
	store = make([]byte, 0)
	br(n-1, n, []byte{'('})
	variants := make([]string, len(store)/n/2)
	for i := 0; i < len(store)/n/2; i++ {
		variants[i] = string(store[i*n*2 : i*n*2+n*2])
	}

	return variants
}

func br(n, cl int, left []byte) {
	if n == 0 {
		store = append(store, append(left, getClosing(cl)...)...)
		return
	}
	br(n-1, cl, append(left, '('))
	for i := 0; i < cl-n; i++ {
		l := append(left, getClosing(i+1)...)
		br(n-1, cl-i-1, append(l, '('))
	}
}

func getClosing(n int) []byte {
	return closingBrackets[:n]
}
