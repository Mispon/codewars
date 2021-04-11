package kyu_6

import "strings"

func DuplicateCount(str string) (c int) {
	charsMap := make(map[rune]int)
	for _, char := range strings.ToLower(str) {
		if charsMap[char]++; charsMap[char] == 2 {
			c++
		}
	}
	return
}
