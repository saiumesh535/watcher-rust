package main

import (
	"fmt"
	"net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {
	fmt.Println("Hey!!");
    fmt.Fprintf(w, "hello, from Rust, Hyderabad\n")
}

func helloYe(w http.ResponseWriter, req *http.Request) {
	fmt.Println("Hey!!");
    fmt.Fprintf(w, "hello, from welcome\n")
}

func headers(w http.ResponseWriter, req *http.Request) {
	fmt.Println("Hey!!");
    for name, headers := range req.Header {
        for _, h := range headers {
            fmt.Fprintf(w, "%v: %v\n", name, h)
        }
    }
}


func main() {
    http.HandleFunc("/", hello)
    http.HandleFunc("/headers", helloYe)
    if err := http.ListenAndServe("localhost:4321", nil); err != nil {
		fmt.Println("gey!!")
	}
}