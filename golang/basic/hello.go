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

	nums := [5]int{1, 3, 5, 7, 9}

	// 创建切片，概念上类似于C++的vector
	var slice []int = nums[0:2]
	fmt.Println(slice)  // [1 3]
	nums[0] = 2  // 改动切片就改动了原数组
	fmt.Println(slice)  // [2 3]
	fmt.Println(nums)  // [2 3 5 7 9]
}
