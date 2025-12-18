package main

import (
	"encoding/binary"
	"fmt"
	// "io"
	"os"
)

func main() {
	fmt.Printf("workin'\n")

	path := "/tmp/collisionmodel1.hkt.PS3"
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	hkx := make([]byte, 16)
	binary.Read(file, binary.BigEndian, &hkx)
	header(file)
}

func header(file *os.File) {
	// https://reshax.com/topic/198-havok-middleware/
	var h struct {
		Magic1                         uint32
		Magic2                         uint32
		UserTag                        uint32
		Version                        uint32
		PointerSize                    uint8
		LittleEndian                   bool
		ReuseBaseClassPadding          bool
		EmptyBaseClassOptimization     bool
		NumSections                    int32
		ContentsSectionIndex           int32
		ContentsSectionOffset          int32
		ContentsClassNameSectionIndex  int32
		ContentsClassNameSectionOffset int32
		ContentsVersion                [0x10]byte
	}
	binary.Read(file, binary.BigEndian, &h)

	fmt.Println(h)
}
