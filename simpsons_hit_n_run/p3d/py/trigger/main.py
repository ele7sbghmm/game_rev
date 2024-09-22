import numpy as np
import json
from struct import unpack


def behead(func):
    def inner(br):
        br.read(12)
        func(br)
    return inner


@behead
def colvec(br):
    return unpack('3f', br.read(12))


def obbox(br, *args):
    l1, l2, l3 = unpack('3f', br.read(12))
    pos = colvec(br)

    v1 = colvec(br)
    v2 = colvec(br)
    v3 = colvec(br)

    return {
        'l1': l1,
        'l2': l2,
        'l3': l3,
        'pos': pos,
        'v1': v1,
        'v2': v2,
        'v3': v3,
    }


def locator(br, *args):
    name_len, = unpack('b', br.read(1))
    name, = unpack(f'{name_len}s', br.read(name_len))
    ttype, num_elements = unpack('2I', br.read(8))
    elements = br.read(num_elements * 4)
    translation = unpack('3f', br.read(12))
    num_of_triggers, = unpack("I", br.read(4))
    triggers = [trigger(br) for _ in range(num_of_triggers)]

    return {
        'name': name.split(b'\x00')[0].decode(),
        "ttype": ttype,
        "elements": elements.split(b'\x00')[0].decode(),
        "translation": translation,
        "triggers": triggers,
    }


@behead
def trigger(br, *args):
    name_len, = unpack('b', br.read(1))
    name, = unpack(f'{name_len}s', br.read(name_len))
    sphere_rect, = unpack('I', br.read(4))
    scale = unpack('3f', br.read(12))
    transformation_matrix = unpack('16f', br.read(0x40))
    translation = transformation_matrix[12:15]
    angle = np.arctan2(transformation_matrix[2], transformation_matrix[0])

    return {
        'name': name.split(b'\x00')[0].decode(),
        'sphere_rect': sphere_rect,
        'scale': scale,
        # 'transformation_matrix': transformation_matrix,
        'translation': translation,
        'angle': angle,
    }


def fence(br, *args):
    br.read(12)  # wall header
    start = unpack('3f', br.read(12))
    end = unpack('3f', br.read(12))
    normal = unpack('3f', br.read(12))

    return {
        "start":  list(start),
        "end":    list(end),
        "normal": list(normal),
    }


def skip(br, chunk_size, *args):
    br.read(chunk_size)


def p3d(*args): return None


def match_id(id):
    match id:
        case 0xff443350: return p3d
        # case 0x03000005: return locator
        # case 0x03f00007: return fence
        case 0x07010004: return obbox
        # case 0x07010002: return sphere
        # case 0x07010003: return cylinder
        case _: return skip


def main(br):
    ls = []
    n = b'a'
    while n := br.read(12):
        id, data_size, chunk_size = unpack('3I', n)
        if not (chunk := match_id(id)(br, chunk_size - 12)):
            continue
        ls.append(chunk)
    print(json.dumps(ls))


if __name__ == '__main__':
    # with open("../../files/fences/flandersHouse.p3d", "rb") as file:
    # with open("../../files/terras/L7_TERRA.p3d", "rb") as file:
    with open("../../files/l7z1.p3d", "rb") as file:
        main(file)
