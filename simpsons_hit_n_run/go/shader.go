package main

import (
	"bytes"
	"encoding/binary"
)

func ShaderLoader(c *ChunkHandler, buffer *bytes.Buffer) {
	const ID_SHADER = 0x00011000
	const ID_SHADER_DEFINITION = 0x00011001
	const ID_TEXTURE_PARAM = 0x00011002
	const ID_INT_PARAM = 0x00011003
	const ID_FLOAT_PARAM = 0x00011004
	const ID_COLOUR_PARAM = 0x00011005
	const ID_VECTOR_PARAM = 0x00011006
	const ID_MATRIX_PARAM = 0x00011007
	if c.beginChunk() != ID_SHADER {
		return
	}

	var nameLen uint8
	binary.Read(c.r, binary.LittleEndian, &nameLen)
	name := make([]byte, nameLen)
	binary.Read(c.r, binary.LittleEndian, &name)

	var version uint32
	binary.Read(c.r, binary.LittleEndian, &version)
	var shaderNameLen uint8
	binary.Read(c.r, binary.LittleEndian, &shaderNameLen)
	shaderName := make([]byte, shaderNameLen)
	binary.Read(c.r, binary.LittleEndian, &shaderName)

	var hasTranslucency uint32
	binary.Read(c.r, binary.LittleEndian, &hasTranslucency)
	var vertexNeeds uint32
	binary.Read(c.r, binary.LittleEndian, &vertexNeeds)
	var vertexMask uint32
	binary.Read(c.r, binary.LittleEndian, &vertexMask)

	var count uint32
	binary.Read(c.r, binary.LittleEndian, &count)

	for c.chunksRemaining() {
		switch c.beginChunk() {
		case ID_SHADER_DEFINITION:
			{
			}
		case ID_TEXTURE_PARAM:
			{

			}
		case ID_INT_PARAM:
			{
			}
		case ID_FLOAT_PARAM:
			{
			}
		case ID_COLOUR_PARAM:
			{
			}
		case ID_VECTOR_PARAM:
			{
			}
		case ID_MATRIX_PARAM:
			{
			}
		}
		c.endChunk()
	}

}
