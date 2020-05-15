package main

import (
	"example/lol"
	"fmt"
	"net/http"
)


func main() {
    http.HandleFunc("/s", lol.Hello)
    if err := http.ListenAndServe("localhost:4321", nil); err != nil {
		fmt.Println("gey!!")
	}
}
