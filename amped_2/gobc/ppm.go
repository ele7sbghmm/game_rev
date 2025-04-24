package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
)

type PPM7 struct {
	name   string
	width  uint32
	height uint32
	packed []uint8
}

func (p *PPM7) FromFormat(file io.Reader, format uint32) {
	resFmt := unpackResourceFormat(format)
	p.width = 1 << resFmt.WidthShift
	p.height = 1 << resFmt.HeightShift

	switch resFmt.Format {
	case 0x6:
		p.name = fmt.Sprintf("%s_RGBA8", p.name)

		p.packed = make([]uint8, p.width*p.height*4)
		binary.Read(file, binary.LittleEndian, &p.packed)
		return
	case 0xc:
		p.name = fmt.Sprintf("%s_BC1", p.name)

		var bc1 BC1
		for range p.height / 4 {
			qprow := make([]QuadPixel, p.width/4)
			for i := range qprow {
				bc1.Parse(file)
				qprow[i] = bc1.ToQuadPixel()
			}
			for py := range 4 {
				for x := range p.width / 4 {
					qp := qprow[x]
					r0 := qp[py*4+0].Pack()
					r1 := qp[py*4+0].Pack()
					r2 := qp[py*4+0].Pack()
					r3 := qp[py*4+0].Pack()
					p.packed = append(p.packed, r0[:]...)
					p.packed = append(p.packed, r1[:]...)
					p.packed = append(p.packed, r2[:]...)
					p.packed = append(p.packed, r3[:]...)
				}
			}
		}
		return
	case 0xe:
		p.name = fmt.Sprintf("%s_BC2", p.name)

		var bc2 BC2
		for range p.height / 4 {
			qprow := make([]QuadPixel, p.width/4)
			for i := range qprow {
				bc2.Parse(file)
				qprow[i] = bc2.ToQuadPixel()
			}
			for py := range 4 {
				for x := range p.width / 4 {
					qp := qprow[x]
					r0 := qp[py*4+0].Pack()
					r1 := qp[py*4+0].Pack()
					r2 := qp[py*4+0].Pack()
					r3 := qp[py*4+0].Pack()
					p.packed = append(p.packed, r0[:]...)
					p.packed = append(p.packed, r1[:]...)
					p.packed = append(p.packed, r2[:]...)
					p.packed = append(p.packed, r3[:]...)
				}
			}
		}
		return
	case 0xf:
		p.name = fmt.Sprintf("%s_BC3", p.name)

		var bc3 BC3
		for range p.height / 4 {
			qprow := make([]QuadPixel, p.width/4)
			for i := range qprow {
				bc3.Parse(file)
				qprow[i] = bc3.ToQuadPixel()
			}
			for py := range 4 {
				for x := range p.width / 4 {
					qp := qprow[x]
					r0 := qp[py*4+0].Pack()
					r1 := qp[py*4+0].Pack()
					r2 := qp[py*4+0].Pack()
					r3 := qp[py*4+0].Pack()
					p.packed = append(p.packed, r0[:]...)
					p.packed = append(p.packed, r1[:]...)
					p.packed = append(p.packed, r2[:]...)
					p.packed = append(p.packed, r3[:]...)
				}
			}
		}
		return
	default:
		return
	}
}

func (p *PPM7) FromQuadPixelArray(qpa []QuadPixel) {
	for y := range p.height / 4 {
		for qy := range 4 {
			for x := range p.width / 4 {
				qp := qpa[y*p.width/4+x]
				for qx := range 4 {
					c := qp[qy*4+qx].Pack()
					p.packed = append(p.packed, c[:]...)
				}
			}
		}
	}
}

func (p *PPM7) WriteToFile() {
	out, _ := os.Create(fmt.Sprintf("%s.ppm", p.name))
	defer out.Close()
	header := fmt.Sprintf("P7\nWIDTH %d\nHEIGHT %d\nMAXVAL 255\nDEPTH 4\nTUPL_TYPE RGB_ALPHA\nENDHDR\n", p.width, p.height)
	out.WriteString(header)
	if p.packed[0] == 0x0a || p.packed[0] == 0x20 {
		p.packed[0] += 1
	}
	_, err := out.Write(p.packed)

	fmt.Printf("%s.ppm ", p.name)
	if err != nil {
		fmt.Printf("failed\n")
	} else {
		fmt.Printf("succeeded\n")
	}
}
func CalcQuadPixelsFromWidthHeight(w uint32, h uint32) uint32 {
	return w * h / 16
}
