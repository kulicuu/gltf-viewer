use bitflags::bitflags;


use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;


use cgmath::{Matrix, Matrix4, Vector3, Vector4};
use cgmath::prelude::*;



pub struct Shader {
    pub id: u32,
    uniform_location_cache: HashMap<&'static str, i32>
}


bitflags! {
    /// Flags matching the defines in the PBR shader
    pub struct ShaderFlags: u16 {
        // vertex shader + fragment shader
        const HAS_NORMALS           = 1;
        const HAS_TANGENTS          = 1 << 1;
        const HAS_UV                = 1 << 2;
        const HAS_COLORS            = 1 << 3;

        // fragment shader only
        const USE_IBL               = 1 << 4;
        const HAS_BASECOLORMAP      = 1 << 5;
        const HAS_NORMALMAP         = 1 << 6;
        const HAS_EMISSIVEMAP       = 1 << 7;
        const HAS_METALROUGHNESSMAP = 1 << 8;
        const HAS_OCCLUSIONMAP      = 1 << 9;
        const USE_TEX_LOD           = 1 << 10;
    }
}

pub struct PbrShader {
    // pub shader: Shader,
    // pub flags: ShaderFlags,
    // pub uniforms: PbrUniformLocations,
}


impl PbrShader {
    pub fn new(
        // flags: ShaderFlags
    )
    -> Self 
    {
        
        
        
        // let mut shader = Shader::from_source(
        //     include_str!("shaders/pbr-vert.glsl"),
        //     include_str!("shaders/pbr-frag.glsl"),
        //     &flags.as_strings());

        // // NOTE: shader debug version
        // // let mut shader = Shader::new(
        // //     "src/shaders/pbr-vert.glsl",
        // //     "src/shaders/pbr-frag.glsl",
        // //     &flags.as_strings());

        // let uniforms = unsafe {
        //     let uniforms = PbrUniformLocations {
        //         u_MVPMatrix: shader.uniform_location("u_MVPMatrix"),
        //         u_ModelMatrix: shader.uniform_location("u_ModelMatrix"),
        //         u_Camera: shader.uniform_location("u_Camera"),

        //         u_LightDirection: shader.uniform_location("u_LightDirection"),
        //         u_LightColor: shader.uniform_location("u_LightColor"),

        //         u_AmbientLightColor: shader.uniform_location("u_AmbientLightColor"),
        //         u_AmbientLightIntensity: shader.uniform_location("u_AmbientLightIntensity"),

        //         u_DiffuseEnvSampler: shader.uniform_location("u_DiffuseEnvSampler"),
        //         u_SpecularEnvSampler: shader.uniform_location("u_SpecularEnvSampler"),
        //         u_brdfLUT: shader.uniform_location("u_brdfLUT"),

        //         u_BaseColorSampler: shader.uniform_location("u_BaseColorSampler"),
        //         u_BaseColorFactor: shader.uniform_location("u_BaseColorFactor"),

        //         u_NormalSampler: shader.uniform_location("u_NormalSampler"),
        //         u_NormalScale: shader.uniform_location("u_NormalScale"),

        //         u_EmissiveSampler: shader.uniform_location("u_EmissiveSampler"),
        //         u_EmissiveFactor: shader.uniform_location("u_EmissiveFactor"),

        //         u_MetallicRoughnessSampler: shader.uniform_location("u_MetallicRoughnessSampler"),
        //         u_MetallicRoughnessValues: shader.uniform_location("u_MetallicRoughnessValues"),

        //         u_OcclusionSampler: shader.uniform_location("u_OcclusionSampler"),
        //         u_OcclusionStrength: shader.uniform_location("u_OcclusionStrength"),

        //         u_ScaleDiffBaseMR: shader.uniform_location("u_ScaleDiffBaseMR"),
        //         u_ScaleFGDSpec: shader.uniform_location("u_ScaleFGDSpec"),
        //         u_ScaleIBLAmbient: shader.uniform_location("u_ScaleIBLAmbient"),
        //     };

        //     shader.use_program();
        //     shader.set_int(uniforms.u_BaseColorSampler, 0);
        //     shader.set_int(uniforms.u_NormalSampler, 1);
        //     shader.set_int(uniforms.u_EmissiveSampler, 2);
        //     shader.set_int(uniforms.u_MetallicRoughnessSampler, 3);
        //     shader.set_int(uniforms.u_OcclusionSampler, 4);

        //     shader.set_vec3(uniforms.u_LightColor, 5.0, 5.0, 5.0);
        //     // TODO!: optional minus on z
        //     shader.set_vec3(uniforms.u_LightDirection, 0.0, 0.5, 0.5);

        //     shader.set_vec3(uniforms.u_AmbientLightColor, 1.0, 1.0, 1.0);
        //     shader.set_float(uniforms.u_AmbientLightIntensity, 0.2);

        //     uniforms
        // };

        Self {
            // shader,
            // flags,
            // uniforms
        }
    }
}
