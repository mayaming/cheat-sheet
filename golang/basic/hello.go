package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello, world")

	// 创建数组
	var cities [3]string
	cities[0] = "Beijing"
	cities[1] = "Shanghai"
	cities[2] = "Guangzhou"
	fmt.Println(cities[0], cities[1], cities[2])  // Beijing Shanghai Guangzhou
	fmt.Println(cities)  // [Beijing Shanghai Guangzhou]
}
