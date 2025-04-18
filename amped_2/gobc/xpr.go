package main

import "fmt"

type PgfSizes struct {
	TotalFileSize   uint32
	TotalDataSize   uint32
	TextureDataSize uint32
	NumTextures     uint32
	ShaderDataSize  uint32
}
type XprResource struct {
	Common uint32
	Data   uint32
	Lock   uint32
	Format uint32
	Size   uint32
}
type XprResourceFormat struct {
	Dma            uint32
	Dimensionality uint32
	Format         uint32
	Levels         uint32
	WidthShift     uint32
	HeightShift    uint32
	DepthShift     uint32
}

func unpackResourceFormat(u32 uint32) XprResourceFormat {
	return XprResourceFormat{
		Dma:            (u32 & 0b00000000_00000000_00000000_00001111) >> 0,
		Dimensionality: (u32 & 0b00000000_00000000_00000000_11110000) >> 4,
		Format:         (u32 & 0b00000000_00000000_11111111_00000000) >> 8,
		Levels:         (u32 & 0b00000000_00001111_00000000_00000000) >> 16,
		WidthShift:     (u32 & 0b00000000_11110000_00000000_00000000) >> 20,
		HeightShift:    (u32 & 0b00001111_00000000_00000000_00000000) >> 24,
		DepthShift:     (u32 & 0b11110000_00000000_00000000_00000000) >> 28,
	}
}

type ImageFormat int

const (
	IMGFMT_RGBA8 ImageFormat = 6
	IMGFMT_BC1   ImageFormat = 0xc
	IMGFMT_BC2   ImageFormat = 0xe
)

func (f ImageFormat) GetType() any {
	switch f {
	case IMGFMT_RGBA8:
		var out RGB565
		return out
	case IMGFMT_BC1:
		var out BC1
		return out
	case IMGFMT_BC2:
		var out BC2
		return out
	default:
		panic(fmt.Sprintf("format %02x not implemented", uint8(f)))
	}
}

func GetType(f int) any {
	switch f {
	case 6:
		var out RGB565
		return out
	case 0xc:
		var out BC1
		return out
	case 0xe:
		var out BC2
		return out
	default:
		panic(fmt.Sprintf("format %02x not implemented", uint8(f)))
	}
}
