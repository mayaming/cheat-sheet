package main

import (
	"fmt"
)

func main() {
	var gdp map[string]int32
	gdp = make(map[string]int32)
	gdp["Beijing"] = 65536
	gdp["Shanghai"] = 72000
	gdp["Guangzhou"] = 32000

	fmt.Println(gdp)  // map[Beijing:65536 Guangzhou:32000 Shanghai:72000]
}