from __future__ import annotations
import numpy as np

class Vec3f:
    def __init__(self, x: float, y: float, z: float) -> Vec3f:
        self.x = x
        self.y = y
        self.z = z
    def __repr__(self) -> str:
        return f"x: {self.x:.2f} | y: {self.y:.2f} | z: {self.z:.2f}"
    def to_obj(self) -> str:
        return f"v {self.x:.6f} {self.y:.6f} {self.z:.6f}\n"
    def scale(self, other: Vec3f) -> Vec3f:
        return Vec3f(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        )
    def scale_float(self, f: float) -> Vec3f:
        return Vec3f(
            self.x * f,
            self.y * f,
            self.z * f,
        )
    def add(self, other: Vec3f) -> Vec3f:
        return Vec3f(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    def add_float(self, f: float) -> Vec3f:
        return Vec3f(
            self.x + f,
            self.y + f,
            self.z + f,
        )
    def magnitude(self) -> float:
        return np.sqrt(
            np.square(self.x)
            + np.square(self.y)
            + np.square(self.z)
        )
    def normalize(self) -> Vec3f:
        factor = self.magnitude()
        return Vec3f(self.x/factor, self.y/factor, self.z/factor)
