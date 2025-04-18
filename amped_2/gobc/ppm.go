package main

import (
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

	fmt.Println(format)

	switch resFmt.Format {
	case 0x6:
		return
	case 0xc:
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
	case 0xe:
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
	out, _ := os.Create(p.name)
	defer out.Close()
	header := fmt.Sprintf("P7\nWIDTH %d\nHEIGHT %d\nMAXVAL 255\nDEPTH 4\nTUPL_TYPE RGB_ALPHA\nENDHDR\n", p.width, p.height)
	out.WriteString(header)
	fmt.Println(header)
	out.Write(p.packed)
}
func CalcQuadPixelsFromWidthHeight(w uint32, h uint32) uint32 {
	return w * h / 16
}
