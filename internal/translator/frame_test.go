package translator

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestFrame_Encode(t *testing.T) {
	id, err := NewChannelID()
	assert.Nil(t, err)

	frame := NewFrame(id, 1, []byte("Hello World"), true)

	expected := id[:]
	expected = append(expected, 0, 1, 0, 0, 0, 11)
	expected = append(expected, []byte("Hello World")...)
	expected = append(expected, 1)

	encoded, err := frame.Encode()
	assert.Nil(t, err)
	assert.NotNil(t, encoded)
	assert.Equal(t, expected, encoded)
}
