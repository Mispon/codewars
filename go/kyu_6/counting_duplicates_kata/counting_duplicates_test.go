package kyu_6

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_DuplicateCount(t *testing.T) {
	test0 := DuplicateCount("abcd")
	assert.Equal(t, 0, test0)

	test1 := DuplicateCount("aabbbc123qwe")
	assert.Equal(t, 2, test1)

	test2 := DuplicateCount("abcda")
	assert.Equal(t, 1, test2)

	test3 := DuplicateCount("Indivisibilities")
	assert.Equal(t, 2, test3)
}
