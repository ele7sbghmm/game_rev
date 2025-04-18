package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
)

func main() {
	path := "data/NZ1.pgf"

	file, _ := os.Open(path)
	defer file.Close()

	var version float32
	var delimiter uint32
	var sizes PgfSizes

	binary.Read(file, binary.LittleEndian, &version)
	binary.Read(file, binary.LittleEndian, &delimiter)
	binary.Read(file, binary.LittleEndian, &sizes)
	binary.Read(file, binary.LittleEndian, &delimiter)

	resources := make([]XprResource, sizes.NumTextures)
	for i := range len(resources) {
		binary.Read(file, binary.LittleEndian, &resources[i])
	}

	textureDataOffset := 4 + 4 + 0x14 + 4 + sizes.NumTextures*0x14
	for i, res := range resources {
		file.Seek(int64(textureDataOffset+res.Data), io.SeekStart)
		var ppm PPM7
		ppm.name = fmt.Sprintf("ppm/%03x.ppm", i)
		ppm.FromFormat(file, res.Format)

		fmt.Printf("%d w%d h%d done", i, ppm.width, ppm.height)
		if ppm.packed == nil {
			continue
		}

		ppm.WriteToFile()
	}

	str := "workin'"

	fmt.Println(str)
	fmt.Println(sizes.NumTextures)
}
