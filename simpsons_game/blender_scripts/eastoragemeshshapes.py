import bpy, struct


def ea(br, o, n):
    br.seek(o, 0)
    vs, fs = struct.unpack('>24xI8xI', br.read(0x28))
    
    br.read(0xa8)
    v = [struct.unpack('>3f4x', br.read(0x10)) for _ in range(vs)]
    f = [struct.unpack('>3I', br.read(0xc)) for _ in range(fs)]
    
    m = bpy.data.meshes.new('')
    m.from_pydata(v, [], f)
    m.update()
    
    bpy.context.collection.objects.link(
        bpy.data.objects.new(str(n), mesh)
    )


with open('/tmp/collisionmodel1.hkt.PS3', 'rb') as br:
    ea(br, 0x40e30, 0)

