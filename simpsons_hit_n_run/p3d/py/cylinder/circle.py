import numpy as np

from vec import Vec3f

TAU = np.pi * 2

def draw(radius: float, wires: int) -> list[Vec3f]:
    theta = 0.0
    l: list[Vec3f] = list()
    axis = Vec3f(0.0, 1.0, 0.0)

    while theta <= TAU:
        rs = radius * np.sin(theta)
        rc = radius * np.cos(theta)

        v = Vec3f(rs, 0.0, rc)

        s = np.sin(theta)
        ass = axis.scale_float(s)
        # acc = axis.scale_float(rc)
        print(f"{s= }")
        print(f"{ass= }")
        # print(f"{acc= }")
        print(f"{v.normalize().scale_float(radius)= }")
        v = v.add(ass)
        # v = v.add(acc)
        print(f"{v= }")

        l.append(v.normalize().scale_float(radius))

        theta += TAU/wires

    return l

