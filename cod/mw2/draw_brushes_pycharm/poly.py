import numpy as np

from clipmap import cplane_s

def clip(plane, array):
    new_a = []
    dp = 2

    for i, _ in enumerate(array):
        v1 = array[i]
        v2 = array[(i + 1) % len(array)]

        d1 = np.dot(v1, plane.n) - plane.d
        d2 = np.dot(v2, plane.n) - plane.d
        # d1 = round(np.dot(v1, plane.n) - plane.d, dp)
        # d2 = round(np.dot(v2, plane.n) - plane.d, dp)

        # if d1 <= 0:
        if round(d1, dp) <= 0:
            # if d2 <= 0:
            if round(d2, dp) <= 0:
                new_a.append(v2)
            else:
                t = d1 / (d1 - d2)
                sect = v1 + t * (v2 - v1)
                new_a.append(sect)
        # elif d2 <= 0:
        elif round(d2, dp) <= 0:
            t = d1 / (d1 - d2)
            sect = v1 + t * (v2 - v1)
            new_a.append(sect)
            new_a.append(v2)

    return new_a


def plane_basis(plane):
    n = plane.n / np.linalg.norm(plane.n)
    if abs(n[0]) < .9:
        t = np.array([1, 0, 0])
    else:
        t = np.array([0, 1, 0])
    u = np.cross(n, t)
    u /= np.linalg.norm(u)
    v = np.cross(n, u)

    return u, v


def poly(plane, h):
    u, v = plane_basis(plane)
    c = plane.n * plane.d

    return [
        c + u*h + v*h,
        c - u*h + v*h,
        c - u*h - v*h,
        c + u*h - v*h,
    ]


def planes_to_poly(ob, planes, brush_bounds):
    size = 1_000_000

    bounds_planes = boundsToPlanes(brush_bounds)
    clipping_planes = [*planes, *bounds_planes]

    polies = [poly(clipping_planes[i], size) for i in range(len(clipping_planes))]

    for i, ply in enumerate(polies):
        tmp = ply
        for p in clipping_planes:
            tmp = clip(p, tmp)

        ob.trifan(tmp)


def boundsToPlanes(bounds):
    mx, my, mz, hx, hy, hz = bounds
    x, y, z, nx, ny, nz = mx + hx, my + hy, mz + hz, mx - hx, my - hy, mz - hz

    bound_planes = [
        cplane_s(1, 0, 0, x, 0),
        cplane_s(0, 1, 0, y, 0),
        cplane_s(0, 0, 1, z, 0),
        cplane_s(-1, 0, 0, -nx, 0),
        cplane_s(0, -1, 0, -ny, 0),
        cplane_s(0, 0, -1, -nz, 0),
    ]

    return bound_planes