package main

import (
	"fmt"
)

func main() {
	nums := [5]int{1, 3, 5, 7, 9}

	// 创建切片，概念上类似于C++的vector
	var slice []int = nums[0:2]
	fmt.Println(slice)  // [1 3]
	nums[0] = 2  // 改动切片就改动了原数组
	fmt.Println(slice)  // [2 3]
	fmt.Println(nums)  // [2 3 5 7 9]

	var gdp map[string]int32
	gdp = make(map[string]int32)
	gdp["Beijing"] = 65536
	gdp["Shanghai"] = 72000
	gdp["Guangzhou"] = 32000

	fmt.Println(gdp)  // map[Beijing:65536 Guangzhou:32000 Shanghai:72000]
}