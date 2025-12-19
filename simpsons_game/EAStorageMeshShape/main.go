package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		return
	}

	path := os.Args[1]
	// path := "/tmp/collisionmodel1.hkt.PS3"
	outdir := path[:len(path)-8]

	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	cno := peeku(f, 0x64) + 0x10
	ea := findEA(f, int64(cno))

	dso := peeku(f, 0x94) + 0x10
	cmo := peeku(f, 0xa0) + dso
	eof := peeku(f, 0xa4) + dso

	f.Seek(int64(cmo), io.SeekStart)

	var cmt struct {
		O  uint32
		F  uint32
		So uint32
	}

	fmt.Printf("eof: %x", eof)
	var eas []int64
	for {
		pos, _ := f.Seek(0, io.SeekCurrent)
		fmt.Printf("pos: %x\n", pos)

		if pos+0xc > int64(eof) {
			break
		}

		// f.Seek(0x293c4, io.SeekStart)
		binary.Read(f, binary.BigEndian, &cmt)
		if int32(cmt.O) == -1 {
			break
		}
		if cmt.So == ea {
			fmt.Println(cmt.So, ea)
			eas = append(eas, int64(cmt.O+dso))
		}
	}
	fmt.Println(eas)
	for _, nea := range eas {
		fmt.Printf("nea: %x\n", nea)
		n, obj := ea2obj(f, int64(nea))
		err := os.MkdirAll(outdir, 0755)
		if err != nil {
			fmt.Println("Error creating directory:", err)
			return
		}
		p := fmt.Sprintf("%s/%s.obj", outdir, n)

		of, err := os.Create(p)
		if err != nil {
			panic(err)
		}
		defer of.Close()

		_, err = of.WriteString(obj)
		if err != nil {
			panic(err)
		}

		fmt.Println(obj)

	}
}

func ea2obj(f *os.File, o int64) (string, string) {
	var s strings.Builder

	np := o - 0x80
	f.Seek(np, io.SeekStart)
	n := cstr(f)
	n = strings.ReplaceAll(n, "|", "_")

	var vn uint32
	f.Seek(o, io.SeekStart)
	f.Seek(0x18, io.SeekCurrent)
	binary.Read(f, binary.BigEndian, &vn)

	var fn uint32
	f.Seek(0x8, io.SeekCurrent)
	binary.Read(f, binary.BigEndian, &fn)

	f.Seek(0xa8, io.SeekCurrent)
	var v struct {
		X float32
		Y float32
		Z float32
		C float32
	}
	for i := 0; i < int(vn); i++ {
		binary.Read(f, binary.BigEndian, &v)
		fmt.Fprintf(&s, "v %.6f %.6f %.6f\n", v.X, v.Y, v.Z)
	}

	s.WriteString("\n")

	var ff struct {
		A uint32
		B uint32
		C uint32
	}
	for i := 0; i < int(fn); i++ {
		binary.Read(f, binary.BigEndian, &ff)
		fmt.Fprintf(&s, "f %d %d %d\n", ff.A+1, ff.B+1, ff.C+1)
	}

	return n, s.String()
}

func findEA(f *os.File, o int64) uint32 {
	f.Seek(o, io.SeekStart)
	for {
		var flag int8
		f.Seek(4, io.SeekCurrent)
		binary.Read(f, binary.LittleEndian, &flag)
		if flag != 9 {
			break
		}
		p, _ := f.Seek(0, io.SeekCurrent)
		s := cstr(f)

		if s == "EAStorageMeshShape" {
			return uint32(p - o)
		}
	}
	panic("didn't find EAStoreageMeshShape string")
}

func peeku(f *os.File, o int64) uint32 {
	f.Seek(o, io.SeekStart)

	var u uint32
	binary.Read(f, binary.BigEndian, &u)
	return u
}
func cstr(f *os.File) string {
	var buf []byte
	b := make([]byte, 1)

	for {
		_, err := f.Read(b)
		if err != nil {
			panic(err)
		}

		if b[0] == 0 {
			break
		}
		buf = append(buf, b[0])
	}

	return string(buf)
}

func pp(f *os.File, s string) {
	c, _ := f.Seek(0, io.SeekCurrent)
	fmt.Printf("%s: %x\n", s, c)
}
