package main

import (
	"encoding/binary"
	"io"
)

type Chunk struct {
	startIndex int64
	chunkId    uint32
	dataSize   int64
	chunkSize  int64
}
type ChunkHandler struct {
	r        io.ReadSeeker
	stackTop uint32
	stack    []Chunk
}

func newChunkHandler(r io.ReadSeeker) ChunkHandler {
	s := make([]Chunk, 32)
	return ChunkHandler{r, 0, s}
}

func (c *ChunkHandler) beginChunk() uint32 {
	cur := c.stack[c.stackTop]
	start := cur.startIndex + cur.dataSize
	pos := c.getPosition()
	if pos < start {
		c.r.Seek(start-pos, io.SeekCurrent)
	}

	c.stackTop++
	c.stack[c.stackTop].startIndex = c.getPosition()

	var tmp uint32
	binary.Read(c.r, binary.LittleEndian, &c.stack[c.stackTop].chunkId)
	binary.Read(c.r, binary.LittleEndian, &tmp)
	c.stack[c.stackTop].dataSize = int64(tmp)
	binary.Read(c.r, binary.LittleEndian, &tmp)
	c.stack[c.stackTop].chunkSize = int64(tmp)

	return c.stack[c.stackTop].chunkId
}

func (c *ChunkHandler) p3dChunk() {
	var chunk Chunk
	var tmp uint32
	chunk.startIndex = c.getPosition()
	binary.Read(c.r, binary.LittleEndian, &chunk.chunkId)
	binary.Read(c.r, binary.LittleEndian, &tmp)
	chunk.dataSize = int64(tmp)
	binary.Read(c.r, binary.LittleEndian, &tmp)
	chunk.chunkSize = int64(tmp)

	c.stack[0] = chunk
}

func (c *ChunkHandler) endChunk() {
	chunk := c.stack[c.stackTop]
	c.r.Seek(chunk.startIndex+int64(chunk.chunkSize), io.SeekStart)
	c.stackTop--
}

func (c *ChunkHandler) getCurrentId() uint32 {
	return c.stack[c.stackTop].chunkId
}

func (c *ChunkHandler) getPosition() int64 {
	pos, _ := c.r.Seek(0, io.SeekCurrent)
	return pos
}

func (c *ChunkHandler) chunksRemaining() bool {
	chunk := c.stack[c.stackTop]
	return (c.getPosition() < chunk.startIndex+int64(chunk.chunkSize)) && (chunk.dataSize < chunk.chunkSize)
}
