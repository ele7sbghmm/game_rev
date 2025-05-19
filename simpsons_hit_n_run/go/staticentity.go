package main

import (
	"bytes"
	"encoding/binary"
)

func readStaticEntity(ch *ChunkHandler, buffer *bytes.Buffer) {
	var stringLen uint8
	binary.Read(ch.r, binary.LittleEndian, &stringLen)
	name := make([]byte, stringLen)
	binary.Read(ch.r, binary.LittleEndian, &name)

	var version uint32
	binary.Read(ch.r, binary.LittleEndian, &version)
	var hasAlpha uint32
	binary.Read(ch.r, binary.LittleEndian, &hasAlpha)

	for ch.chunksRemaining() {

		if ch.beginChunk() == ID_MESH {
			readMesh(ch, buffer)
		}
		ch.endChunk()
	}
}

func readMesh(ch *ChunkHandler, buffer *bytes.Buffer) {
	var stringLen uint8
	binary.Read(ch.r, binary.LittleEndian, &stringLen)
	name := make([]byte, stringLen)
	binary.Read(ch.r, binary.LittleEndian, &name)

	var version uint32
	binary.Read(ch.r, binary.LittleEndian, &version)
	var nPrimGroup uint32
	binary.Read(ch.r, binary.LittleEndian, &nPrimGroup)

	for ch.chunksRemaining() {
		switch ch.beginChunk() {
		case ID_PRIMGROUP:
			{
				readPrimGroup(ch, buffer)
			}
		}
		ch.endChunk()
	}
}

func readPrimGroup(ch *ChunkHandler, buffer *bytes.Buffer) {
	var version uint32
	binary.Read(ch.r, binary.LittleEndian, &version)

	var nameLen uint8
	binary.Read(ch.r, binary.LittleEndian, &nameLen)
	name := make([]byte, nameLen)
	binary.Read(ch.r, binary.LittleEndian, &name)

	var mPrimType uint32
	var mVertexFormat uint32
	var mVertexCount uint32
	var mIndexCount uint32
	var mMatrixCount uint32
	binary.Read(ch.r, binary.LittleEndian, &mPrimType)
	binary.Read(ch.r, binary.LittleEndian, &mVertexFormat)
	binary.Read(ch.r, binary.LittleEndian, &mVertexCount)
	binary.Read(ch.r, binary.LittleEndian, &mIndexCount)
	binary.Read(ch.r, binary.LittleEndian, &mMatrixCount)

	var count uint32
	var positionData [][12]byte
	var normalData [][12]byte
	var indexData []uint32
	var colorData [][4]uint8
	var uvData [][8]byte
	for ch.chunksRemaining() {
		switch ch.beginChunk() {
		case ID_POSITIONLIST:
			{
				binary.Read(ch.r, binary.LittleEndian, &count)
				positionData = make([][12]byte, count)
				binary.Read(ch.r, binary.LittleEndian, &positionData)
			}
		case ID_NORMALLIST:
			{
				binary.Read(ch.r, binary.LittleEndian, &count)
				normalData = make([][12]byte, count)
				binary.Read(ch.r, binary.LittleEndian, &normalData)
			}
		case ID_COLOURLIST:
			{
				binary.Read(ch.r, binary.LittleEndian, &count)
				colorData = make([][4]uint8, count)
				binary.Read(ch.r, binary.LittleEndian, &colorData)
			}
		case ID_MULTICOLOURLIST:
			{
			}
		case ID_UVLIST:
			{
				binary.Read(ch.r, binary.LittleEndian, &count)
				var channel uint32
				binary.Read(ch.r, binary.LittleEndian, &channel)
				uvData = make([][8]byte, count)
				binary.Read(ch.r, binary.LittleEndian, &uvData)
			}
		case ID_INDEXLIST:
			{
				binary.Read(ch.r, binary.LittleEndian, &count)
				indexData = make([]uint32, count)
				binary.Read(ch.r, binary.LittleEndian, &indexData)
			}
		case ID_WEIGHTLIST:
			{
			}
		case ID_MATRIXLIST:
			{
			}
		}
		ch.endChunk()
	}

	type PRIMTYPE uint32
	const (
		TRIS       PRIMTYPE = 0
		TRISTRIPS  PRIMTYPE = 1
		LINES      PRIMTYPE = 2
		LINESTRIPS PRIMTYPE = 3
		POINTS     PRIMTYPE = 4
	)

	tris := indexData
	switch mPrimType {
	case uint32(TRISTRIPS):
		{
			tris = make([]uint32, (len(indexData)-2)*3)
			for i := range len(indexData) - 2 {
				if i%2 == 0 {
					tris[i*3+0] = indexData[i+0]
					tris[i*3+1] = indexData[i+1]
					tris[i*3+2] = indexData[i+2]
				} else {
					tris[i*3+0] = indexData[i+0]
					tris[i*3+2] = indexData[i+1]
					tris[i*3+1] = indexData[i+2]
				}
			}
		}
	}

	if len(normalData) == 0 {
		normalData = make([][12]byte, len(positionData))
		for i := range normalData {
			normalData[i] = [12]byte{0, 0, 0, 0, 0, 0, 0x80, 0x3f, 0, 0, 0, 0}
		}
	}
	for _, index := range tris {
		buffer.Write(positionData[index][:])
		// buffer.Write(normalData[index][:])
		buffer.Write(colorData[index][:])
		buffer.Write(uvData[index][:])
	}
}
