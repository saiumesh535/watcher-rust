package lol

import (
	"fmt"
	"net/http"
)

// Hello -- slow
func Hello(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "hello, from, Hyderabad\n")
}
