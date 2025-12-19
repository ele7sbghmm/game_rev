package main

import (
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"unsafe"
)

func main() {
	if len(os.Args) < 1 {
		panic("")
	}

	pathIn := os.Args[1]

	f, err := os.Open(pathIn)
	if err != nil {
		panic("")
	}
	defer f.Close()

	///// //// //
	///  parse pointers

	var hkxSize uint32 = 0x10

	var classNamesOffset uint32
	f.Seek(0x64, io.SeekStart)
	binary.Read(f, binary.BigEndian, &classNamesOffset)
	classNamesOffset += hkxSize

	// var eaStringOffset uint32
	// f.Seek()

	var dataSectionOffset uint32
	f.Seek(0x94, io.SeekStart)
	binary.Read(f, binary.BigEndian, &dataSectionOffset)
	dataSectionOffset += hkxSize

	var classMappingOffset uint32
	f.Seek(0xa0, io.SeekStart)
	binary.Read(f, binary.BigEndian, &classMappingOffset)
	classMappingOffset += dataSectionOffset

	var endOfFile uint32
	binary.Read(f, binary.BigEndian, &endOfFile)
	endOfFile += dataSectionOffset

	///// //// //
	///  parse classnames

	var nameMeta struct {
		Hash  uint32
		Flags uint8
	}
	classNames := make(map[uint32]string)

	f.Seek(int64(classNamesOffset), io.SeekStart)

	for {
		binary.Read(f, binary.BigEndian, &nameMeta)
		if nameMeta.Flags != 0x9 {
			break
		}

		nameOffset, _ := f.Seek(0, io.SeekCurrent)
		nameOffset -= int64(classNamesOffset)

		name := readCstr(f)
		classNames[uint32(nameOffset)] = name
	}

	///// //// //
	///  parse classmappings

	type ClassMapping struct {
		DataOffset   uint32
		Flags        uint32
		StringOffset uint32
	}
	var classMapping ClassMapping
	var classMappingArray []ClassMapping

	_, err = f.Seek(int64(classMappingOffset), io.SeekStart)
	if err != nil {
		panic("")
	}

	for {
		cursor, _ := f.Seek(0, io.SeekCurrent)
		if endOfFile < uint32(cursor)+uint32(unsafe.Sizeof(classMapping)) {
			break
		}

		binary.Read(f, binary.BigEndian, &classMapping)
		if int32(classMapping.DataOffset) == -1 {
			break
		}

		classMappingArray = append(classMappingArray, classMapping)
	}

	///// //// //
	///

	var eaStorageMeshShapeOffset uint32
	for o, n := range classNames {
		if n == "EAStorageMeshShape" {
			eaStorageMeshShapeOffset = o
			break
		}
	}

	var eaStorageMeshShapePointers []uint32
	for _, cm := range classMappingArray {
		if cm.StringOffset == eaStorageMeshShapeOffset {
			fmt.Println("Found EAStorageMeshShape at index:", cm.DataOffset)
			eaStorageMeshShapePointers = append(eaStorageMeshShapePointers, cm.DataOffset+dataSectionOffset)
		}
	}

	///// //// //
	///  parse EAStorageMeshShape instances

	type Vertex = [4]float32
	type Face = [3]uint32

	type EAStorageMeshShapeInstance struct {
		Vertices []Vertex
		Faces    []Face
	}
	var eaStorageMeshShapeInstances []EAStorageMeshShapeInstance
	var vertexCount uint32
	var faceCount uint32
	var vertex Vertex
	var face Face

	for _, ptr := range eaStorageMeshShapePointers {
		var vertices []Vertex
		var faces []Face

		f.Seek(int64(ptr), io.SeekStart)
		f.Seek(0x18, io.SeekCurrent) // skip
		binary.Read(f, binary.BigEndian, &vertexCount)

		f.Seek(8, io.SeekCurrent)
		binary.Read(f, binary.BigEndian, &faceCount)

		f.Seek(0xa8, io.SeekCurrent)
		pos, _ := f.Seek(0, io.SeekCurrent)
		fmt.Println("pos:", pos)
		for i := 0; i < int(vertexCount); i++ {
			binary.Read(f, binary.BigEndian, &vertex)
			vertices = append(vertices, vertex)
		}
		for i := 0; i < int(faceCount); i++ {
			binary.Read(f, binary.BigEndian, &face)
			faces = append(faces, face)
		}
		eaStorageMeshShapeInstances = append(eaStorageMeshShapeInstances, EAStorageMeshShapeInstance{
			Vertices: vertices,
			Faces:    faces,
		})
	}

	var vCount uint32 = 1
	vString := ""
	fString := ""
	for _, inst := range eaStorageMeshShapeInstances {
		for _, v := range inst.Vertices {
			vString += fmt.Sprintf("v %.6f %.6f %.6f\n", v[0], v[1], v[2])
		}
		for _, f := range inst.Faces {
			fString += fmt.Sprintf("f %d %d %d\n", f[0]+vCount, f[1]+vCount, f[2]+vCount)
		}
		vCount += uint32(len(inst.Vertices))
	}

	obj, err := os.Create(os.Args[1] + ".obj")
	if err != nil {
		panic(err)
	}

	defer obj.Close()

	obj.WriteString(vString)
	obj.WriteString(fString)
}

func readCstr(f *os.File) string {
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
