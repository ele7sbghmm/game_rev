import sys
import numpy as np

from vec import Vec3f
from circle import draw
from obj import circle2obj

if __name__ == "__main__":
    circle: list[Vec3f] = draw(int(sys.argv[1]), int(sys.argv[2]))

    with open("/tmp/py.obj", "wt") as out:
        out.write(circle2obj(circle))

