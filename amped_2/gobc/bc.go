package bc

import "fmt"

type RGB565 struct {
	r int
	g int
	b int
}
type BC1 struct {
	c0          RGB565
	c1          RGB565
	c2          RGB565
	c3          RGB565
	lookupTable []int
}
