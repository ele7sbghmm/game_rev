package main

import (
	"bytes"
	"fmt"
	"os"
	// "path/filepath"
)

func main() {
	base := "/Users/im/Public/kod/mujhe/data/shar/shar_zip/art"
	pathIns := []string{
		// "L4_l1_gens_69Shape_016.p3d",
		"l7z1.p3d",
		// "L1_TERRA.p3d",
		// "l1z1.p3d",
		// "l1r1.p3d",
		// "l1z2.p3d",
		// "l1r2.p3d",
		// "l1z3.p3d",
		// "l1r3.p3d",
		// "l1z4.p3d",
		// "l1r4a.p3d",
		// "l1r4b.p3d",
		// "l1z6.p3d",
		// "l1r6.p3d",
		// "l1z7.p3d",
		// "l1r7.p3d",
	}
	// dirOut := "/Users/im/Public/kod/mujhe/noclip.website/data/sharTexture/"

	var buffer bytes.Buffer
	for _, p := range pathIns {
		pathIn := fmt.Sprintf("%s/%s", base, p)
		// sectorName := filepath.Base(pathIn)

		if file, err := os.Open(pathIn); err != nil {
			fmt.Printf("%s failed to open", pathIn)
			defer file.Close()
		} else {
			defer file.Close()

			ch := newChunkHandler(file)
			ch.p3dChunk()

			const ID_FENCE = 0x03F0_0007
			const ID_INTERSECT = 0x03F0_0003
			for ch.chunksRemaining() {
				ch.beginChunk()
				TextureLoader(&ch, &buffer)
				// if ch.beginChunk() == ID_STATICENTITY {
				// 	readStaticEntity(&ch, &buffer)
				// }
				// if ch.beginChunk() == ID_FENCE {
				// 	readFence(&ch, &buffer)
				// }
				// if ch.beginChunk() == ID_INTERSECT { readIntersect(&ch, &buffer) }
				ch.endChunk()
			}
		}

		// noExt := sectorName[2 : len(sectorName)-len(filepath.Ext(sectorName))]
		// outPath := fmt.Sprintf("", dirOut)
		// out, _ := os.Create(outPath)
		// defer out.Close()

		// out.Write(buffer.Bytes())
		// buffer.Reset()
	}

}
