import bpy, struct, math


def ea(br, o, n):
    br.seek(o - 128, 0)
    s = br.read(128)
    s = s.strip(b'\x00')
    s = str(s)
    s = s.split('|')[-1]
    
    vs, fs = struct.unpack('>24xI8xI', br.read(0x28))

    br.read(0xa8)
    v = [struct.unpack('>3f4x', br.read(0x10)) for _ in range(vs)]
    f = [struct.unpack('>3I', br.read(0xc)) for _ in range(fs)]

    m = bpy.data.meshes.new('')
    m.from_pydata(v, [], f)
    m.update()
    
    obj = bpy.data.objects.new(f'{str(n)}_{s}', m)
    obj.rotation_euler[0] = math.radians(90)

    bpy.context.collection.objects.link(obj)


def parse_class_names(br, o):
    def read_name(br):
        s = ''

        br.read(4)
        if br.read(1)[0] == 9:
            while (b := br.read(1)[0]) != 0:
                s += chr(b)
        return s

    names = dict()

    br.seek(o, 0)
    while (name := read_name(br)) != '':
        offs = br.tell() - o - len(name) - 1
        names[offs] = name

    return names


def parse_class_mappings(br, o, e, dso, class_names):
    class_mappings = {}

    br.seek(o, 0)

    while br.tell() + 0xc <= e:
        offset, flags, name_offset = struct.unpack('>3i', br.read(0xc))
        if offset == -1 or name_offset == -1:
            break
        if name_offset not in class_names.keys():
            continue

        name = class_names[name_offset]
        if name not in class_mappings.keys():
            class_mappings[name] = [offset + dso]
        else:
            class_mappings[name].append(offset + dso)

    return class_mappings


def main(p):
    with open(p, 'rb') as br:
        hdr = 0x10

        class_names_offset = struct.unpack('>100xI', br.read(100 + 4))[0] + hdr
        data_section_offset = struct.unpack('>44xI', br.read(44 + 4))[0] + hdr
        class_mapping_offset = struct.unpack('>8xI', br.read(8 + 4))[0] + data_section_offset
        end_of_file = struct.unpack('>I', br.read(4))[0] + data_section_offset

        class_names = parse_class_names(br, class_names_offset)
        class_mappings = parse_class_mappings(br, class_mapping_offset, end_of_file, data_section_offset, class_names)

        for i, offset in enumerate(class_mappings['EAStorageMeshShape']):
            ea(br, offset, i)


if __name__ == '__main__':
    main('/tmp/collisionmodel1.hkt.PS3 2')

