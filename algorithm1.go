package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

var closingBrackets []byte

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
	store := make([]string, 0)
	br(n-1, n, []byte{'('}, &store)

	return store
}

func br(n, cl int, left []byte, store *[]string) {
	if n == 0 {
		*store = append(*store, string(append(left, getClosing(cl)...)))
		return
	}
	br(n-1, cl, append(left, '('), store)
	for i := 0; i < cl-n; i++ {
		l := append(left, getClosing(i+1)...)
		br(n-1, cl-i-1, append(l, '('), store)
	}
}

func getClosing(n int) []byte {
	return closingBrackets[:n]
}
