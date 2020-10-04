package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello, world")

	// 创建数组
	var cities [4]string
	cities[0] = "Beijing"
	cities[1] = "Shanghai"
	cities[2] = "Guangzhou"
	cities[3] = "Shenzhen"
	fmt.Println(cities[0], cities[1], cities[2], cities[3])
	fmt.Println(cities)

	primes := [6]int{2, 3, 5, 7, 11, 13}
	fmt.Println(primes)

	// 创建切片
	var singleDigitPrimes []int = primes[0:4]
	fmt.Println(singleDigitPrimes)
}
