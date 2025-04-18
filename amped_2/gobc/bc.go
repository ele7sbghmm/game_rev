package main

import (
	"encoding/binary"
	"io"
)

type RGB565 struct {
	r uint8
	g uint8
	b uint8
}

func (c *RGB565) ToRGBA8() RGBA8 {
	return RGBA8{
		r: c.r,
		g: c.g,
		b: c.b,
		a: 0xff,
	}
}

func (c *RGB565) Parse(file io.Reader) {
	var raw uint16
	binary.Read(file, binary.LittleEndian, &raw)

	c.r = uint8((raw & 0b_11111000_00000000) >> 11 * 8)
	c.g = uint8((raw & 0b_00000111_11100000) >> 5 * 4)
	c.b = uint8((raw & 0b_00000000_00011111) >> 0 * 8)
}

type BC1 struct {
	c0          RGB565
	c1          RGB565
	c2          RGB565
	c3          RGB565
	lookupTable [16]uint8
}

func (bc1 *BC1) Parse(file io.Reader) {
	var c0 RGB565
	var c1 RGB565
	var c2 RGB565
	var c3 RGB565
	c0.Parse(file)
	c1.Parse(file)

	var r uint32
	var g uint32
	var b uint32
	if c1.r > c0.r && c1.g > c0.g && c1.b > c0.b {
		c3.r = 0
		c3.g = 0
		c3.b = 0

		r = uint32(c0.r)
		g = uint32(c0.g)
		b = uint32(c0.b)
		r += uint32(c1.r)
		g += uint32(c1.g)
		b += uint32(c1.b)
		r /= 2
		g /= 2
		b /= 2
		c2.r = uint8(r)
		c2.g = uint8(g)
		c2.b = uint8(b)
	} else {
		r = uint32(c0.r)
		g = uint32(c0.g)
		b = uint32(c0.b)
		r *= 2
		g *= 2
		b *= 2
		r += uint32(c1.r)
		g += uint32(c1.g)
		b += uint32(c1.b)
		r /= 3
		g /= 3
		b /= 3
		c2.r = uint8(r)
		c2.g = uint8(g)
		c2.b = uint8(b)

		r = uint32(c1.r)
		g = uint32(c1.g)
		b = uint32(c1.b)
		r *= 2
		g *= 2
		b *= 2
		r += uint32(c0.r)
		g += uint32(c0.g)
		b += uint32(c0.b)
		r /= 3
		g /= 3
		b /= 3
		c3.r = uint8(r)
		c3.g = uint8(g)
		c3.b = uint8(b)
	}

	var raw uint32
	binary.Read(file, binary.LittleEndian, &raw)

	var lookupTable [16]uint8
	for i := range len(lookupTable) {
		lookupTable[i] = uint8(raw & 0b11)
		raw >>= 2
	}
	bc1.c0 = c0
	bc1.c1 = c1
	bc1.c2 = c2
	bc1.c3 = c3
	bc1.lookupTable = lookupTable
}

func (b *BC1) ToQuadPixel() QuadPixel {
	colorTable := [4]RGBA8{
		b.c0.ToRGBA8(),
		b.c1.ToRGBA8(),
		b.c2.ToRGBA8(),
		b.c3.ToRGBA8(),
	}

	var qp QuadPixel
	for i, index := range b.lookupTable {
		qp[i] = colorTable[index]
	}
	return qp
}

type BC2 struct {
	alphaTable [16]uint8
	bc1        BC1
}

func (bc2 *BC2) Parse(file io.Reader) {
	var alphaTable [16]uint8
	var raw uint64
	binary.Read(file, binary.LittleEndian, &raw)
	for i := range alphaTable {
		alphaTable[i] = uint8(raw & 0b1111)
		raw = raw >> 4
	}

	var bc1 BC1
	bc1.Parse(file)

	bc2.alphaTable = alphaTable
	bc2.bc1 = bc1
}

func (b *BC2) ToQuadPixel() QuadPixel {
	colorTable := [4]RGBA8{
		b.bc1.c0.ToRGBA8(),
		b.bc1.c1.ToRGBA8(),
		b.bc1.c2.ToRGBA8(),
		b.bc1.c3.ToRGBA8(),
	}

	var qp QuadPixel
	for i := range qp {
		qp[i] = colorTable[b.bc1.lookupTable[i]]
		qp[i].a = b.alphaTable[i]
	}
	return qp
}

type RGBA8 struct {
	r uint8
	g uint8
	b uint8
	a uint8
}

func (c *RGBA8) Pack() [4]uint8 {
	return [4]uint8{c.r, c.g, c.b, c.a}
}

type QuadPixel [16]RGBA8
