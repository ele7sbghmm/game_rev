package main

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
	"io"
	"os"
)

type Vec3 struct {
	X float32 `json:"x"`
	Y float32 `json:"y"`
	Z float32 `json:"z"`
}

type Fence struct {
	Start  Vec3 `json:"start"`
	End    Vec3 `json:"end"`
	Normal Vec3 `json:"normal"`
}

func p3d(path string) []Fence {
	fences := make([]Fence, 0, 10_000)

	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	p3dHeader := make([]byte, 12)
	binary.Read(file, binary.LittleEndian, &p3dHeader)
	fmt.Println(p3dHeader)

	for {
		chunk := struct {
			Id        uint32
			DataSize  uint32
			ChunkSize uint32
		}{}

		err := binary.Read(file, binary.LittleEndian, &chunk)
		if err != nil {
			if err == io.EOF {
				break
			}
			panic(err)
		}

		// pos, err := file.Seek(0, io.SeekCurrent)
		// if err != nil {
		// 	panic(err)
		// }
		// fmt.Println(" >>>> ", pos, " >> ", chunk.Id, chunk.DataSize, chunk.ChunkSize)

		if chunk.Id != 0x03f0_0007 {
			dump := make([]byte, chunk.ChunkSize-12)
			file.Read(dump)

			continue
		}

		wallHeader := make([]byte, 12)
		err = binary.Read(file, binary.LittleEndian, &wallHeader)
		if err != nil {
			if err == io.EOF {
				fmt.Println("eof reading wall header")
			}
		}

		var fence Fence
		err = binary.Read(file, binary.LittleEndian, &fence)
		if err != nil {
			if err == io.EOF {
				fmt.Println("eof reading fence")
			}
		}

		// fmt.Println(fence.Start)

		fences = append(fences, fence)
	}

	return fences
}

func writeJson(path string, data interface{}) error {
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()

	encoder := json.NewEncoder(file)
	encoder.SetIndent("", "  ")

	err = encoder.Encode(data)
	return err
}

func main() {
	start := Vec3{X: 3., Y: -2., Z: 12.}
	start2 := Vec3{X: 1000., Y: -1000., Z: 12000.}
	end := Vec3{X: 30., Y: -20., Z: 120.}
	normal := Vec3{X: 1., Y: -0., Z: 0.}
	f := []Fence{
		{Start: start, End: end, Normal: normal},
		{Start: start2, End: end, Normal: normal},
	}

	jsonBytes, err := json.Marshal(f)
	if err != nil {
		panic(err)
	}

	jsonStr := string(jsonBytes)

	fmt.Println(jsonStr)

	fences := p3d("/tmp/L2_TERRA.p3d")
	writeJson("/tmp/json.json", fences)

}
