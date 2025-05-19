package main

import (
	"bytes"
	"encoding/binary"
	"math"
)

func readFence(c *ChunkHandler, buffer *bytes.Buffer) {
	if c.beginChunk() == 0x0300_0000 {
		var sx, sy, sz [4]byte
		binary.Read(c.r, binary.LittleEndian, &sx)
		binary.Read(c.r, binary.LittleEndian, &sy)
		binary.Read(c.r, binary.LittleEndian, &sz)
		var ex, ey, ez [4]byte
		binary.Read(c.r, binary.LittleEndian, &ex)
		binary.Read(c.r, binary.LittleEndian, &ey)
		binary.Read(c.r, binary.LittleEndian, &ez)
		var n [12]byte
		binary.Read(c.r, binary.LittleEndian, &n)

		h, l := make([]byte, 4), make([]byte, 4)
		binary.LittleEndian.PutUint32(l, math.Float32bits(float32(-10.0)))
		binary.LittleEndian.PutUint32(h, math.Float32bits(float32(10.0)))
		color := [4]byte{255, 0, 0, 255}

		p0 := bytes.Join([][]byte{sx[:], h[:], sz[:], n[:], color[:]}, []byte{})
		p1 := bytes.Join([][]byte{sx[:], l[:], sz[:], n[:], color[:]}, []byte{})
		p2 := bytes.Join([][]byte{ex[:], l[:], ez[:], n[:], color[:]}, []byte{})
		p3 := bytes.Join([][]byte{ex[:], h[:], ez[:], n[:], color[:]}, []byte{})

		t0 := bytes.Join([][]byte{p0[:], p1[:], p2[:]}, []byte{})
		t1 := bytes.Join([][]byte{p0[:], p2[:], p3[:]}, []byte{})

		buffer.Write(t0)
		buffer.Write(t1)
	}
	c.endChunk()
}

func readIntersect(c *ChunkHandler, buffer *bytes.Buffer) {
	var numIndices uint32
	binary.Read(c.r, binary.LittleEndian, &numIndices)
	indices := make([]uint32, numIndices)
	binary.Read(c.r, binary.LittleEndian, &indices)

	var numVertices uint32
	binary.Read(c.r, binary.LittleEndian, &numVertices)
	vertices := make([][12]byte, numVertices)
	binary.Read(c.r, binary.LittleEndian, &vertices)

	var numNormals uint32
	binary.Read(c.r, binary.LittleEndian, &numNormals)
	normals := make([][12]byte, numNormals)
	binary.Read(c.r, binary.LittleEndian, &normals)

	var terrainType []uint8
	terrainType = make([]uint8, numNormals)

	const ID_TERRAIN_TYPE = 0x0300000E
	for c.chunksRemaining() {
		if c.beginChunk() == ID_TERRAIN_TYPE {
			var version uint32
			binary.Read(c.r, binary.LittleEndian, &version)
			var size uint32
			binary.Read(c.r, binary.LittleEndian, &size)
			binary.Read(c.r, binary.LittleEndian, &terrainType)
		}
		c.endChunk()
	}
	colors := [][4]byte{
		{0x47, 0x48, 0x4c, 0xff}, // road
		{0x3f, 0x9b, 0x0b, 0xff}, // grass
		{0xf6, 0xdc, 0xbd, 0xff}, // sand
		{0x72, 0x7e, 0x8b, 0xff}, // gravel
		{0x7a, 0xa2, 0xc4, 0xff}, // water
		{0xc3, 0x84, 0x52, 0xff}, // wood
		{0x4a, 0x6e, 0x78, 0xff}, // metal
		{0xa1, 0x78, 0x5c, 0xff}, // dirt
	}

	for i, index := range indices {
		tri := int(math.Floor(float64(i) / float64(3)))

		buffer.Write(vertices[index][:])
		buffer.Write(normals[tri][:])
		buffer.Write(colors[terrainType[tri]][:])
	}
}
