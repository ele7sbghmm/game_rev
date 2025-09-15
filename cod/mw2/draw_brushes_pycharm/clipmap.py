import numpy as np
from struct import unpack
from dataclasses import dataclass

# conv = lambda f, b, o: f + ((o & 0xfffffff) - 1) - ((b & 0xfffffff) - 1)
conv = lambda o: (o & 0xfffffff) - 1

def find_null(r):
    cur = r.tell()

    n = 1
    while r.read(1) != b'\x00':
        n += 1

    r.seek(cur)
    return n


class clipMap_t:
    brushCount = 0x469f
    def __init__(self, r):
        self.planes = cplane_s.parse_array(r, 0x70f6dc, 0x41cc)
        self.sides = cbrushside_t.parse_array(r, 0x3a3179c, 0x8ebe)
        self.brushes = cbrush_t.parse_array(r, 0x3c32c7c, clipMap_t.brushCount)
        self.brushBounds = parse_bounds_array(r, 0x3cd1ad8, clipMap_t.brushCount)
        self.brushContents = parse_int_array(r, 0x3d3b9c0, clipMap_t.brushCount)

        # self.leafbrushes = parse_leafbrushes(r, 0x3af585e, 0x77e9)
        # self.leafBrushNodes = parse_cLeafBrushNode_s_array(r, 0x3b04830, 0x3c47)
        # self.leafs = cLeaf_t.parse_array(r, 0x3ad59e6, 0xcc3)
        # self.cmodels = cmodel_t.parse_array(r, 0x3c3250c, 0x1c)
        # self.clipmaterials = ClipMaterial.parse_array(r, 0x3a28491, 0x679)
        # self.aabbTrees = CollisionAabbTree.parse_array(r, 0x, 0x)

        # self.verts = parse_verts(r, 0x03b4fdfc, 0x55a5)
        # self.triIndices = parse_triIndices(r, 0x03b901b8, 0x81f4)
        print()


def parse_int_array(r, p, n):
    r.seek(p)

    return list(unpack(f'{n}I', r.read(n * 4)))


def parse_bounds_array(r, p, n):
    r.seek(p)

    a = []
    for _ in range(n):
        mx, my, mz, hx, hy, hz = unpack('6f', r.read(24))
        a.append([mx, my, mz, hx, hy, hz])

    return a


def parse_verts(r, p, n):
    r.seek(p)

    return [unpack('3f', r.read(12)) for _ in range(n)]


def parse_triIndices(r, p, n):
    r.seek(p)

    return [unpack('3h', r.read(6)) for _ in range(n)]


@dataclass
class CollisionAabbTree:
    midPoint: list[np.ndarray]
    materialIndex: int
    childCount: int
    midPoint: list[np.ndarray]
    u: int#CollisionAabbTreeIndex

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for _ in range(n):
            ax, ay, az, materialIndex, childCount, bx, by, bz, u \
                = unpack('3f2H3fI', r.read(0x2c))
            midPoint = np.array([ax, ay, az])
            halfSize = np.array([bx, by, bz])

            aabbTree = CollisionAabbTree(midPoint, materialIndex, childCount, halfSize, u)
            a.append(aabbTree)

        return a


class cmodel_t:
    size = 0x44

    def __init__(self, bounds, radius, leaf):
        self.bounds = bounds
        self.radius = radius
        self.leaf = leaf

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for i in range(n):
            *bounds, radius = unpack('7f', r.read(28))
            leaf = cLeaf_t.parse(r, None)
            model = cmodel_t(bounds, radius, leaf)
            a.append(model)

        return a


class ClipMaterial:
    size = 0xc

    def __init__(self, name, surfaceFlags, contents):
        self.name = name
        self.surfaceFlags = surfaceFlags
        self.contents = contents

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for i in range(n):
            name_ptr, surfaceFlags, contents = unpack('i2I', r.read(12))
            mat = ClipMaterial(name_ptr, surfaceFlags, contents)
            a.append(mat)

        for mat in a:
            if mat.name == -1:
                n = find_null(r)
                mat.name = r.read(n).decode('utf8').replace('\x00', '')
            else:
                mat.name = ''

        return a


class cLeaf_t:
    size = 0x28

    def __init__(self, firstCollAabbIndex, collAabbCount, brushContents, terrainContents, bounds, leafBrushNode):
        self.firstCollAabbIndex = firstCollAabbIndex
        self.collAabbCount = collAabbCount
        self.brushContents = brushContents
        self.terrainContents = terrainContents
        self.bounds = bounds
        self.leafBrushNode = leafBrushNode

    @staticmethod
    def parse(r, p):
        if p is not None:
            r.seek(p)

        firstCollAabbIndex, collAabbCount, brushContents, terrainContents = unpack('2H2I', r.read(12))
        *bounds, leafBrushNode = unpack('6fi', r.read(28))
        model = cLeaf_t(firstCollAabbIndex, collAabbCount, brushContents, terrainContents, bounds, leafBrushNode + 1)

        return model

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for _ in range(n):
            model = cLeaf_t.parse(r, None)
            a.append(model)

        return a


class cLeafBrushNode_s_leaf:
    size = 0x14

    def __init__(self, leafBrushCount, contents, brushes):
        self.contents = contents
        self.leafBrushCount = leafBrushCount

         ## cLeafBrushNodeData_t ##
        self.brushes = brushes


class cLeafBrushNode_s_internal:
    size = 0x14

    def __init__(self, axis, contents, dist, range_, childOffset):
        self.axis = axis
        self.contents = contents

         ## cLeafBrushNodeData_t ##
        self.dist = dist
        self.range = range_
        self.childOffset = childOffset


class cbrush_t:
    size = 0x24

    def __init__(self, numsides, glassPieceIndex, sides_num, baseAdjacentSide, axialMaterialNum, firstAdjacentSideOffsets, edgeCount):
        self.numsides = numsides
        self.glassPieceIndex = glassPieceIndex
        self.sides = sides_num
        self.baseAdjacentSide = baseAdjacentSide
        self.axialMaterialNum = axialMaterialNum
        self.firstAdjacentSideOffsets = firstAdjacentSideOffsets
        self.edgeCount = edgeCount

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for _ in range(n):
            numsides, glassPieceIndex, sides_ptr, baseAdjacentSide = unpack('2H2I', r.read(12))
            axialMaterialNum = [list(unpack('2H', r.read(4))) for _ in range(3)]
            tmp = list(unpack('12B', r.read(12)))
            firstAdjacentSideOffsets = tmp[:6]
            edgeCount = tmp[6:12]

            sides_num = (sides_ptr - 0x3242457d) // cbrushside_t.size
            brush = cbrush_t(numsides, glassPieceIndex, sides_num, baseAdjacentSide, axialMaterialNum, firstAdjacentSideOffsets, edgeCount)
            a.append(brush)

        return a


class cbrushside_t:
    size = 8

    def __init__(self, plane_num, materialNum, firstAdjacentSideOffset, edgeCount):
        self.plane_num = plane_num
        self.materialNum = materialNum
        self.firstAdjacentSideOffset = firstAdjacentSideOffset
        self.edgeCount = edgeCount

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)
        m = 0xffffffff

        a = []
        for _ in range(n):
            plane_ptr, materialNum, firstAdjacentSideOffset, edgeCount = unpack('Ih2B', r.read(8))
            plane_num = (plane_ptr - 0x3044bb01) // cplane_s.size
            if plane_ptr < m:
                m = plane_ptr
            brushside = cbrushside_t(plane_num, materialNum, firstAdjacentSideOffset, edgeCount)
            a.append(brushside)

        return a


class cplane_s:
    size = 0x14

    def __init__(self, nx, ny, nz, d, t):
        self.n = np.array([nx, ny, nz])
        # self.n = np.array([nx * -1, ny * -1, nz * -1])
        self.d = d
        self.t = t

    @staticmethod
    def parse_array(r, p, n):
        r.seek(p)

        a = []
        for i in range(n):
            x, y, z, d, t = unpack('4fB3x', r.read(0x14))
            plane = cplane_s(x, y, z, d, t)
            a.append(plane)

        return a


def parse_leafbrushes(r, p, n):
    r.seek(p)

    return list(unpack(f'{n}H', r.read(n * 2)))

def parse_cLeafBrushNode_s_array(r, p, n):
    r.seek(p)

    m = 0
    a = []
    for i in range(n):
        axis, leafBrushCount, contents = unpack('BxhI', r.read(8))

        if leafBrushCount > 0:  ### leaf
            brushes_ptr, *_ = unpack('i8x', r.read(12))

            if brushes_ptr == -1:
                brushes = -1
            else:
                cur = r.tell()
                r.seek(brushes_ptr - 0x324e8641 + 0x3af585e)
                brushes = list(unpack(f'{leafBrushCount}H', r.read(leafBrushCount * 2)))
                r.seek(cur)

            a.append(cLeafBrushNode_s_leaf(leafBrushCount, contents, brushes))

        else:  ### internal node
            dist, range_, *childOffset = unpack('2f2H', r.read(12))

            a.append(cLeafBrushNode_s_internal(axis, contents, dist, range_, childOffset))

    for node in a:
        if getattr(node, "brushes", None) == -1:
            n = node.leafBrushCount
            node.brushes = list(unpack(f'{n}H', r.read(n * 2)))

    print('leafBrushCount ', i, m)
    return a
