class Obj:
    def __init__(self):
        self.v = []

    def vert(self, v):
        self.v.append(v)

    def minmax(self, nx, ny, nz, x, y, z):
        self.vert((nx, ny, nz)), self.vert((x,  ny, z)), self.vert((nx, ny, z))
        self.vert((nx, ny, nz)), self.vert((x,  ny, nz)), self.vert((x,  ny, z))
        self.vert((nx, y, nz)), self.vert((nx, y,  z)), self.vert((x,  y, z))
        self.vert((nx, y, nz)), self.vert((x,  y, z)), self.vert((x,  y, nz))

        self.vert((nx, ny, nz)), self.vert((x,  y,  nz)), self.vert((x,  ny, nz))
        self.vert((nx, ny, nz)), self.vert((nx, y,  nz)), self.vert((x,  y,  nz))
        self.vert((nx, ny, z)), self.vert((x,  ny, z)), self.vert((x,  y,  z))
        self.vert((nx, ny, z)), self.vert((x,  y,  z)), self.vert((nx, y,  z))

        self.vert((nx, ny, nz)), self.vert((nx, ny,  z)), self.vert((nx,  y,  z))
        self.vert((nx, ny, nz)), self.vert((nx,  y,  z)), self.vert((nx,  y, nz))
        self.vert((x, ny, nz)), self.vert((x,  y,  z)), self.vert((x, ny,  z))
        self.vert((x, ny, nz)), self.vert((x,  y, nz)), self.vert((x,  y,  z))

    def bounds(self, bounds):
        mx, my, mz, hx, hy, hz = bounds
        x, y, z, nx, ny, nz = mx + hx, my + hy, mz + hz, mx - hx, my - hy, mz - hz

        self.minmax(x, y, z, nx, ny, nz)

    def quad(self, a, b, c, d):
        self.vert(a), self.vert(b), self.vert(c)
        self.vert(a), self.vert(c), self.vert(d)

    def trifan(self, vs):
        for i in range(1, len(vs)-1, 1):
            self.vert(vs[0])
            self.vert(vs[i + 0])
            self.vert(vs[i + 1])

    def write(self, p):
        if len(self.v) < 3:
            return

        with open(f'{p}', 'wt') as f:
            for v in self.v:
                f.write(f'v {v[0]} {v[1]} {v[2]}\n')

            f.write('\n')
            for i in range(1, len(self.v) + 1, 3):#4):
                f.write(f'f {i + 0} {i + 1} {i + 2}\n')
                # f.write(f'f {i + 0} {i + 2} {i + 3}\n')

            print(p)