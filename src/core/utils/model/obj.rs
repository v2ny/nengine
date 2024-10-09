#[derive(Debug)]
pub struct MeshData {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
	pub normals: Vec<f32>,
	pub texcoords: Vec<f32>,
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
        let mut normals = Vec::new();
        let mut texcoords = Vec::new();

        if let Some(model) = models.first() {
            let mesh = &model.mesh;

            vertices = mesh.positions.clone();  // Vertices are already in the positions field
            indices = mesh.indices.clone();     // Indices for the EBO
			normals = mesh.normals.clone();
			texcoords = mesh.texcoords.clone();
        }

        // Return the mesh data
        Ok(MeshData { vertices, indices, normals, texcoords })
    }
}
