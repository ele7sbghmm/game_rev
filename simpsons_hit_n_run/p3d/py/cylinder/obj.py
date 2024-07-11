from vec import Vec3f

def circle2obj(l: list[Vec3f]):
    faces: int = 1
    obj: str = ""

    for v in l:
        obj += v.to_obj()

    return obj

