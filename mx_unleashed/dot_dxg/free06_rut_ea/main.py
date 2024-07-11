from dataclasses import dataclass
from struct import unpack
import io
from ctypes import Structure, c_int, c_float

class Vec3f(Structure):
    _fields_ = [
        ("x", c_float),
        ("y", c_float),
        ("z", c_float),
    ]
    def to_v(self) -> str:
        return f"v {self.x:.6f} {self.y:.6f} {self.z:.6f}\n"
    @staticmethod
    def parse(input: bytes) -> "Vec3f":
        if len(input) < 12: print(input)
        assert len(input) == 12
        return Vec3f(*unpack("3f", input))
    @staticmethod
    def from_fileio(file: io.BytesIO):
        return Vec3f.parse(file.read(12))
    @staticmethod
    def array_to_v(array: bytes) -> str:
        s: str = ""
        for index, _ in enumerate(array):
            if index % 12: continue
            vec = Vec3f.parse(array[index:index+12])
            s += vec.to_v()
        return s


if __name__ == '__main__':
    # ib 1dc ee   vb1 32e 870
    path = "/Users/mair/_kode/_reversing/game_files/mx/xbox_ex2/geom\\free06_rut_e_a.dxg"
    with open(path, "rb") as file: file = file.read()

    vecs = Vec3f.array_to_v(file[0x32e:0x32e+0x870])

    s: str = ""
    array = file[0x152:0x152+0xee*2]
    indices = unpack(f'{len(array)//2}H', array)

    n = 0
    for i, _ in enumerate(indices[:-2]):
        n += 1
        x, y, z = indices[i], indices[i+1], indices[i+2]
        if not x != y != z != x: continue
        if not i % 2: s += f"f {x+1} {z+1} {y+1}\n"
        else:         s += f"f {x+1} {y+1} {z+1}\n"

    print(s)

    print(indices)
    print(f"{len(array)=}")
    print(f"{len(indices)=}")

    print(n)

    with open(
        '/Users/mair/_kode/_smrt/game_rev/mx_unleashed/dot_dxg/free06_rut_ea/solving_indices/ib_0xee_132_234__fixed.obj',
        'wt'
    ) as out:
        out.write(vecs)
        out.write("\n")
        out.write(s)
    
