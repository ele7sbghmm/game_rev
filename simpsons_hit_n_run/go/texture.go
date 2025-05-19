package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"os"
	"strings"
)

func TextureLoader(c *ChunkHandler, buffer *bytes.Buffer) {
	const ID_TEXTURE = 0x00019000
	if c.getCurrentId() != ID_TEXTURE {
		return
	}

	var nameLen uint8
	binary.Read(c.r, binary.LittleEndian, &nameLen)
	name := make([]byte, nameLen)
	binary.Read(c.r, binary.LittleEndian, &name)

	var version uint32
	binary.Read(c.r, binary.LittleEndian, &version)

	var width uint32
	binary.Read(c.r, binary.LittleEndian, &width)
	var height uint32
	binary.Read(c.r, binary.LittleEndian, &height)
	var bpp uint32
	binary.Read(c.r, binary.LittleEndian, &bpp)
	var alphaDepth uint32
	binary.Read(c.r, binary.LittleEndian, &alphaDepth)
	var numMipMaps uint32
	binary.Read(c.r, binary.LittleEndian, &numMipMaps)

	var textureType uint32
	binary.Read(c.r, binary.LittleEndian, &textureType)
	var usage uint32
	binary.Read(c.r, binary.LittleEndian, &usage)
	var priority uint32
	binary.Read(c.r, binary.LittleEndian, &priority)

	const ID_IMAGE = 0x00019001
	const ID_VOLUME_IMAGE = 0x00019004
	const ID_SPRITE = 0x00019005

	for c.chunksRemaining() {
		switch c.beginChunk() {
		case ID_IMAGE:
			{
				ImageLoader(c)
			}
		case ID_VOLUME_IMAGE:
			{
			}
		}
		c.endChunk()
	}
}
func ImageLoader(c *ChunkHandler) {
	var nameLen uint8
	binary.Read(c.r, binary.LittleEndian, &nameLen)
	name := make([]byte, nameLen)
	binary.Read(c.r, binary.LittleEndian, &name)
	var version uint32
	binary.Read(c.r, binary.LittleEndian, &version)

	var width uint32
	binary.Read(c.r, binary.LittleEndian, &width)
	var height uint32
	binary.Read(c.r, binary.LittleEndian, &height)
	var bpp uint32
	binary.Read(c.r, binary.LittleEndian, &bpp)
	var palettized uint32
	binary.Read(c.r, binary.LittleEndian, &palettized)
	var alpha uint32
	binary.Read(c.r, binary.LittleEndian, &alpha)
	var format uint32
	binary.Read(c.r, binary.LittleEndian, &format)

	const ID_IMAGE_DATA = 0x00019002
	const ID_IMAGE_FILENAME = 0x00019003
	for c.chunksRemaining() {
		switch c.beginChunk() {
		case ID_IMAGE_DATA:
			{
				var size uint32
				binary.Read(c.r, binary.LittleEndian, &size)
				imageData := make([]byte, size)
				binary.Read(c.r, binary.LittleEndian, &imageData)

				s := string(name)
				s = strings.ReplaceAll(s, "\x00", "")
				out := fmt.Sprintf("/Users/im/Public/kod/mujhe/noclip.website/data/sharTexture/png/%s.png", s)
				file, err := os.Create(out)
				if err != nil {
					fmt.Printf("failed createing %s", name)
				}
				defer file.Close()
				file.Write(imageData[:])
			}
		case ID_IMAGE_FILENAME:
			{
			}
		}
		c.endChunk()
	}

}
