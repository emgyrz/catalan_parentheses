package main

import (
	"fmt"
	// "strings"
	"time"
	"os"
	"strconv"
)

var closingBrackets string

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
	// fmt.Println(len(variants), strings.Join(variants, " , "))
}


func brackets(n int) []string {
	if n < 1 {
		return nil
	}
	closingBrackets = ""
	for i := 0; i < n; i++ {
		closingBrackets += ")"
	}
	store := make([]string, 0)
	br(n-1, n, "(", &store)

	return store
}

func br(n, cl int, left string, store *[]string) {
	if n == 0 {
		*store = append(*store, left+getClosing(cl))
		return
	}
	br(n-1, cl, left+"(", store)
	for i := 0; i < cl-n; i++ {
		br(n-1, cl-i-1, left+getClosing(i+1)+"(", store)
	}
}

func getClosing(n int) string {
	return closingBrackets[:n]
}
