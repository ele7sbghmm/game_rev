package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
)

type FileEntry struct {
	U1    uint32
	U2    uint32
	U3    uint32
	Size  uint32
	Xsize uint32
	U6    uint32
}

type Header struct {
	Fourcc [4]byte
	Unk1   uint32
	Nfiles uint8
	Unk2   uint8
	Unk3   uint16
	Unk4   uint32
	Offs   uint32
}

func header(file *os.File) []FileEntry {
	var h Header
	binary.Read(file, binary.BigEndian, &h)
	fmt.Println(string(h.Fourcc[:]))
	fmt.Println(h)

	file.Seek(int64(h.Offs), 0)

	entries := make([]FileEntry, h.Nfiles)
	for i := 0; i < int(h.Nfiles); i++ {
		binary.Read(file, binary.BigEndian, &entries[i])
	}

	round(file, 0x800)
	return entries
}

func nstr(file *os.File) string {
	var n uint8
	binary.Read(file, binary.LittleEndian, &n)

	bytes := make([]byte, n)
	binary.Read(file, binary.LittleEndian, &bytes)

	return string(bytes[:])
}

func cur(file *os.File, mes string) {
	offs, _ := file.Seek(0, io.SeekCurrent)
	fmt.Printf(mes+": 0x%x\n", offs)
}

func round(file *os.File, n int64) {
	offs, err := file.Seek(0, io.SeekCurrent)
	if err != nil {
		panic(err)
	}

	if offs%n != 0 {
		diff := n - offs
		_, err := file.Seek(diff, io.SeekCurrent)
		if err != nil {
			panic(err)
		}
	}
}

func refpack(file *os.File, entry FileEntry) {
	var magic uint16
	binary.Read(file, binary.BigEndian, &magic)
	fmt.Printf("magic: %x\n", magic)
	// if magic != 0x10fb {
	// 	panic(fmt.Printf("magic: %d != 0x10fb", magic))
	// }

}

func main() {
	fmt.Println("workin'")

	path := "/tmp/zone18.str"

	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	entries := header(file)
	for _, entry := range entries {
		tmp, _ := file.Seek(0, io.SeekCurrent)
		refpack(file, entry)

		_, err := file.Seek(tmp+int64(entry.Xsize), io.SeekStart)
		if err != nil {
			panic("")
		}
		round(file, 0x800)
	}

}
