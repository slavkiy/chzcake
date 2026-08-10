package main

import (
	"log"

	"github.com/slavkiy/witgo"
)

func main() {
	err := witgo.GeneratePackage(witgo.Config{
		Output:  "./internal/contract",
		Package: "contract",
	}, "./wit")
	if err != nil {
		log.Fatal(err)
	}
}
