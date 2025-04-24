package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"io/fs"
	"os"
)

func main() {
	pgfPath := os.Args[1]

	path := fmt.Sprintf("../../../noclip.website/data/amped2_prototype_sep12/%s", pgfPath)
	fmt.Println(path)

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

		os.MkdirAll(fmt.Sprintf("ppm/%s", pgfPath), fs.ModePerm)
		ppm.name = fmt.Sprintf("ppm/%s/%02x", pgfPath, i)
		ppm.FromFormat(file, res.Format)

		if ppm.packed == nil {
			continue
		}

		ppm.WriteToFile()
	}
}
