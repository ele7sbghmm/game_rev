from dataclasses import dataclass
from struct import unpack
import io

path_to_file = p2f =\
    "/Users/mair/_kode/_smrt/game_rev/mx_unleashed/file_parsing/dot_col/rr_col_01/col__free06_rr_col_01.col"

@dataclass
class V:
    x: float; y: float; z: float
    def to_v(self) -> str:
        return f'v {self.x} {self.y} {self.z}'
    @staticmethod
    def from_bytes(input: bytes) -> 'V':
        assert len(input) == 12
        return V(*unpack('3f', input))
    def from_fileio(file: io.BytesIO) -> 'V':
        return V.from_bytes(file.read(12))
    
@dataclass
class VB_22:
    xyz: V
    unknown: int
    h1: int; h2: int; h3: int 
    


if __name__ == '__main__':
    with open(p2f, 'rb') as file:
        _ = file.read(72)
        _24, _31, _32, _ = unpack('4I', file.read(12))

        first_vbuf = [V.from_fileio(file) for _ in range(_24)]
        _01 = unpack('I', file.read(4))
        second_vbuf = [V.from_fileio(file) for _ in range(_24)]
        third_vbuf = [V.from_fileio(file) for _ in range(_31)]
        forth_vbuf = [V.from_fileio(file) for _ in range(_32)]

