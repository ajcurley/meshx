use std::collections::{HashMap, HashSet};

use crate::geometry::{Aabb, Triangle, Vector3};
use crate::mesh::half_edge::HeMesh;
use crate::mesh::Face;
use crate::spatial::{Octree, SearchMany};

#[derive(Debug, Clone)]
pub struct CartesianMesh {
    config: CartesianMeshConfig,
    geometry: HeMesh,
    geometry_index: Octree<Triangle>,
    blocks: Octree<Aabb>,
    reference_depth: usize,
}

impl CartesianMesh {
    /// Construct a Cartesian mesh generator
    pub fn new(config: CartesianMeshConfig, geometry: HeMesh) -> CartesianMesh {
        CartesianMesh {
            config,
            geometry,
            geometry_index: Octree::<Triangle>::default(),
            blocks: Octree::<Aabb>::default(),
            reference_depth: 0,
        }
    }

    /// Generate the Cartesian mesh
    pub fn generate(&mut self) {
        self.index_geometry();
        self.generate_blocks();
        self.refine_blocks();
        self.generate_boundary();
    }

    /// Index the geometry in an Octree for fast querying.
    fn index_geometry(&mut self) {
        let aabb = self.geometry.aabb();
        let mut geometry_index = Octree::<Triangle>::new(aabb);

        for i in 0..self.geometry.n_faces() {
            let vertices = self.geometry.face_vertices(i);

            if vertices.len() != 3 {
                panic!("geometry must be triangulated");
            }

            let p = self.geometry.vertex(vertices[0]).point();
            let q = self.geometry.vertex(vertices[1]).point();
            let r = self.geometry.vertex(vertices[2]).point();
            let triangle = Triangle::new(p, q, r);

            geometry_index.insert(triangle);
        }

        self.geometry_index = geometry_index;
    }

    /// Initialize the background block mesh with cubic blocks of length base
    /// size that covers the bounding box of the geometry.
    fn generate_blocks(&mut self) {
        let aabb = self.geometry.aabb();
        let length = aabb.halfsize().max();
        let n = (length / self.config.base_size).log2().ceil();
        let reference_depth = (n as usize) + 1;

        let center = aabb.center();
        let halfsize = Vector3::ones() * 2_f64.powf(n) * self.config.base_size;
        let aabb = Aabb::new(center, halfsize);

        let mut blocks = Octree::<Aabb>::new(aabb);
        let mut queue = vec![1];

        while let Some(code) = queue.pop() {
            let node = blocks.node(code);
            let depth = node.depth();

            if node.is_leaf() && depth < reference_depth {
                let mut children = blocks.split(code);
                queue.append(&mut children);
            }
        }

        self.blocks = blocks;
        self.reference_depth = reference_depth;
    }

    /// Refine the background block mesh to the desired refinement level for
    /// each intersecting surface triangle.
    fn refine_blocks(&mut self) {
        println!("Refining blocks");
        let mut queue = self.blocks.leaves();

        while !queue.is_empty() {
            println!("Refinement iteration: {} blocks", queue.len());
            let mut marked = HashSet::new();
            let mut unique = HashSet::new();
            queue.retain(|&c| unique.insert(c));

            let queries: Vec<Aabb> = queue.iter().map(|&c| self.blocks.node(c).aabb()).collect();
            let results = self.geometry_index.search_many(&queries);

            for (i, items) in results.iter().enumerate() {
                let code = queue[i];
                let node = self.blocks.node(code);
                let current_level = node.depth() - self.reference_depth;

                if !items.is_empty() {
                    for j in items.iter() {
                        if let Some(patch) = self.geometry.face(*j).patch() {
                            let name = self.geometry.patch(patch).name();
                            let target_level = self.config.refinement(name);

                            if target_level > current_level {
                                marked.insert(code);
                                break;
                            }
                        }
                    }
                }
            }

            queue.clear();

            for &code in marked.iter() {
                let mut children = self.blocks.split(code);
                queue.append(&mut children);
            }
        }

        println!("Total cells: {}", self.blocks.leaves().len());
    }

    /// Generate the boundary cut-cells
    pub fn generate_boundary(&self) {
        println!("Generating boundary cut-cells");
        let leaves = self.blocks.leaves();
        let queries: Vec<Aabb> = leaves.iter().map(|&c| self.blocks.node(c).aabb()).collect();
        let results = self.geometry_index.search_many(&queries);

        for items in results.iter() {
            if !items.is_empty() {
                println!("-----------------------");
                for index in items.iter() {
                    let neighbors = self.geometry.face_neighbors(*index);

                    for neighbor in neighbors.iter() {
                        if items.contains(neighbor) {
                            println!("Pair found: ({}, {})", index, neighbor);
                        }
                    }
                }

                if items.len() > 2 {
                    return;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CartesianMeshConfig {
    base_size: f64,
    refinements: HashMap<String, usize>,
}

impl CartesianMeshConfig {
    /// Get the refinement level for a patch
    pub fn refinement(&self, patch: &str) -> usize {
        *self.refinements.get(patch).unwrap_or(&0_usize)
    }

    /// Set the refinement level for a patch
    pub fn set_refinement(&mut self, patch: &str, level: usize) {
        self.refinements
            .entry(patch.to_string())
            .and_modify(|l| *l = level)
            .or_insert(level);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cartesian_generate() {
        let path_vehicle =
            "/Users/acurley/projects/github/cfd-toolkit/examples/lmp1/input/vehicle.obj.gz";
        let path_tunnel =
            "/Users/acurley/projects/github/cfd-toolkit/examples/lmp1/input/tunnel.obj.gz";

        let mut geometry = HeMesh::from_obj(&path_vehicle).unwrap();
        let geometry_tunnel = HeMesh::from_obj(&path_tunnel).unwrap();
        geometry.merge(&geometry_tunnel);

        let mut config = CartesianMeshConfig::default();
        config.base_size = 3.2512;
        config.set_refinement("body_cockpit_shell", 8);
        config.set_refinement("body_engine_cover", 8);
        config.set_refinement("body_engine_scoop", 8);
        config.set_refinement("body_front_keel", 8);
        config.set_refinement("body_ls_brake_duct", 8);
        config.set_refinement("body_ls_fender", 8);
        config.set_refinement("body_ls_pod", 8);
        config.set_refinement("body_nose", 8);
        config.set_refinement("body_rear_face", 9);
        config.set_refinement("body_rear_sail", 9);
        config.set_refinement("body_rear_endplates", 9);
        config.set_refinement("body_rear_wing_flap", 10);
        config.set_refinement("body_rear_wing_main_plane", 10);
        config.set_refinement("body_rs_brake_duct", 8);
        config.set_refinement("body_rs_fender", 8);
        config.set_refinement("body_rs_pod", 8);
        config.set_refinement("body_top_deck", 8);
        config.set_refinement("body_windshield", 8);
        config.set_refinement("contact_patch_lf", 11);
        config.set_refinement("contact_patch_lr", 11);
        config.set_refinement("contact_patch_rf", 11);
        config.set_refinement("contact_patch_rr", 11);
        config.set_refinement("rotating_lf_tire", 9);
        config.set_refinement("rotating_lf_wheel", 9);
        config.set_refinement("rotating_lr_tire", 9);
        config.set_refinement("rotating_lr_wheel", 9);
        config.set_refinement("rotating_rf_tire", 9);
        config.set_refinement("rotating_rf_wheel", 9);
        config.set_refinement("rotating_rr_tire", 9);
        config.set_refinement("rotating_rr_wheel", 9);
        config.set_refinement("splitter_flap", 10);
        config.set_refinement("splitter_ls_tire_seal", 10);
        config.set_refinement("splitter_main", 10);
        config.set_refinement("splitter_nose_pillars", 10);
        config.set_refinement("splitter_rs_tire_seal", 10);
        config.set_refinement("underbody_diffuser", 8);
        config.set_refinement("underbody_floor_center", 8);
        config.set_refinement("underbody_keel", 8);
        config.set_refinement("underbody_ls_fender", 8);
        config.set_refinement("underbody_ls_floor_upper", 8);
        config.set_refinement("underbody_ls_rocker", 8);
        config.set_refinement("underbody_rs_fender", 8);
        config.set_refinement("underbody_rs_floor_upper", 8);
        config.set_refinement("underbody_rs_rocker", 8);
        config.set_refinement("underbody_diffuser_ls_strake", 9);
        config.set_refinement("underbody_diffuser_rs_strake", 9);
        config.set_refinement("tunnel_inlet", 2);
        config.set_refinement("tunnel_outlet", 2);
        config.set_refinement("tunnel_farfield", 2);
        config.set_refinement("tunnel_road", 2);
        config.set_refinement("tunnel_road_belt", 8);

        let mut mesh = CartesianMesh::new(config, geometry);
        mesh.generate();

        assert!(false);
    }
}
