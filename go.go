package main

import (
	"fmt"
	// "strings"
	"os"
	"strconv"
)

func main() {
	count, err := strconv.Atoi(os.Args[1])
	if err == nil {
		fmt.Println(count)
	}
	variants := brackets(count)

	fmt.Println(len(variants))
	// fmt.Println(len(variants), strings.Join(variants, " , "))
}


func brackets(n int) []string {
	if n < 1 {
		return nil
	}
	store := make([]string, 0)
	br(n-1, n, "(", &store)

	return store
}

func br(n, cl int, left string, store *[]string) {
	if n == 0 {
		*store = append(*store, left + getClosing(cl))
		return
	}
	br(n-1, cl, left+"(", store)
	for i := 0; i < cl-n; i++ {
		br(n-1, cl-i-1, left+getClosing(i+1)+"(", store)
	}
	//br(n-1, cl-1, left+")(", store)
}

func getClosing(n int) string {
	closing := ""
	for i :=0 ; i < n; i++ {
		closing += ")"
	}

	return closing
}
