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

type Level struct {
	LevelId uint    `json:"levelid"`
	Terra   []Fence `json:"terra"`
	Race1   []Fence `json:"race1"`
	Race2   []Fence `json:"race2"`
	Race3   []Fence `json:"race3"`
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
	levels := []Level{}
	for l := 1; l < 8; l++ {
		level := Level{
			LevelId: uint(l),
			Terra:   []Fence{},
			Race1:   []Fence{},
			Race2:   []Fence{},
			Race3:   []Fence{},
		}

		pathT := fmt.Sprintf("/tmp/art/l%d_terra.p3d", l)
		pathR1 := fmt.Sprintf("/tmp/art/l%d_sr1p.p3d", l)
		pathR2 := fmt.Sprintf("/tmp/art/l%d_sr2p.p3d", l)
		pathR3 := fmt.Sprintf("/tmp/art/l%d_sr3p.p3d", l)

		level.Terra = p3d(pathT)
		level.Race1 = p3d(pathR1)
		level.Race2 = p3d(pathR2)
		level.Race3 = p3d(pathR3)

		levels = append(levels, level)
	}

	pathOut := "/tmp/fences.json"
	writeJson(pathOut, levels)
}
