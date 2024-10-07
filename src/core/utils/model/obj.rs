#[derive(Debug)]
pub struct MeshData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn load(path: &str) -> Result<MeshData, String> {
        // Load the model using tobj
        let (models, _materials) = tobj::load_obj(path, &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        }).map_err(|e| e.to_string())?;

        // Extract vertices and indices from the first model
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        if let Some(model) = models.first() {
            let mesh = &model.mesh;

            vertices = mesh.positions.clone();  // Vertices are already in the positions field
            indices = mesh.indices.clone();     // Indices for the EBO
        }

        // Return the mesh data
        Ok(MeshData { vertices, indices })
    }
}
