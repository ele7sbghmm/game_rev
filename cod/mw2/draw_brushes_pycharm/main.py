import os
import poly, clipmap, obj


def write_brushes(cm, i, n):
    for bit in range(24, 25):
        ob = obj.Obj()

        mask = 1 << bit
        # for j in range(0, 6):
        for j, _ in enumerate(cm.brushes):
            # ob = obj.Obj()

            brush = cm.brushes[j]
            brush_bounds = cm.brushBounds[j]
            contents = cm.brushContents[j]

            if not contents & mask:
                continue

            planes = []
            for side_num in range(brush.numsides):
                side = cm.sides[side_num + brush.sides]
                plane = cm.planes[side.plane_num]
                planes.append(plane)

            poly.planes_to_poly(ob, planes, brush_bounds)

        path = f'/Users/im/Desktop/{mask}.obj'
        os.makedirs(os.path.dirname(path), exist_ok=True)
        ob.write(path)


def main(path):
    with open(path, 'rb') as r:
        cm = clipmap.clipMap_t(r)

        write_brushes(cm, 0, 1)


if __name__ == '__main__':
    main('/Users/im/Desktop/mp_favela.xassets')
