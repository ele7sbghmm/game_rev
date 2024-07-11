#[cfg(test)]

use crate::{Chunk, P3d, Fence, Wall, chunk_paris, red};
use crate::utils::Vec3f;

mod tests {
    #[test]
    fn test_wall() {
        let wall = Wall {
            start:  Vec3f {x: 550.7968, y: 0.0, z: -197.1268},
            end:    Vec3f {x: 550.852, y: 0.0, z: -191.3186},
            normal: Vec3f {x: -0.99995494, y: -0.0, z: 0.009495777}
        };
        let buf = red("/tmp/shar/wall/wall_.p3d");
        
        let (remaining, chunktype) =
            chunk_paris::<()>(buf.as_slice()).unwrap();
        if let ChunkVecs::DataSubs(Chunk::Wall(chunk), _) = chunktype {
            assert_eq!(wall, chunk);
        } else { () }
    }

    #[test]
    fn test_fence() {
        let fence_and_wall = ChunkVecs::DataSubs(
            Chunk::Fence(Fence {}), vec![ChunkVecs::DataSubs(
                Chunk::Wall(Wall {
                    start: Vec3f {x: 550.7968, y: 0.0, z: -197.1268},
                    end: Vec3f {x: 550.852, y: 0.0, z: -191.3186},
                    normal: Vec3f {x: -0.99995494, y: -0.0, z: 0.009495777}
        }), vec![])]);
        let buf = red("/tmp/shar/wall/fence_.p3d");
        
        let (remaining, chunktype) =
            chunk_paris::<()>(buf.as_slice()).unwrap();
        // if let ChunkVecs::DataSubs(Chunk::Fence(fence), v) = chunktype {
        assert_eq!(chunktype, fence_and_wall);
        // } else { () } 
    }

    #[test]
    fn test_p3dfence() {
        let fence_and_wall = ChunkVecs::DataSubs(
            Chunk::P3d(P3d {}), vec![ChunkVecs::DataSubs(
                Chunk::Fence(Fence {}), vec![ChunkVecs::DataSubs(
                    Chunk::Wall(Wall {
                        start: Vec3f {x: 550.7968, y: 0.0, z: -197.1268},
                        end: Vec3f {x: 550.852, y: 0.0, z: -191.3186},
                        normal: Vec3f {x: -0.99995494, y: -0.0, z: 0.009495777}
                    }), vec![])]
        )]);
        let buf = red("/tmp/shar/wall/fence.p3d");
        
        let (remaining, chunktype) =
            chunk_paris::<()>(buf.as_slice()).unwrap();
        // if let ChunkVecs::DataSubs(Chunk::Fence(fence), v) = chunktype {
        assert_eq!(chunktype, fence_and_wall);
        // } else { () } 
    }
}