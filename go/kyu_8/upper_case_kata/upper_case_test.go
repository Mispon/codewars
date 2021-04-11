package kyu_8

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_MakeUpperCase(t *testing.T) {
	helloCase := MakeUpperCase("hello")
	assert.Equal(t, helloCase, "HELLO")
}
