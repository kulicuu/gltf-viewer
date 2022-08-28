use bitflags::bitflags;


use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use std::sync::Arc;


use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as GL, 
    window, AngleInstancedArrays, KeyboardEvent,
    EventTarget, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation,
};

use gloo_console::log;
use wasm_bindgen::prelude::*;

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
    pub shader: Arc<WebGlProgram>,
    pub flags: ShaderFlags,
    pub uniform_locations: PbrUniformLocations,
}

#[allow(non_snake_case)]
pub struct PbrUniformLocations {
    // uniform locations,
    // // TODO!: UBO for matrices, camera, light(s)?
    pub u_MVPMatrix:  WebGlUniformLocation,
    // pub u_ModelMatrix: i32,
    // pub u_Camera: i32,

    // pub u_LightDirection: i32,
    // pub u_LightColor: i32,

    // pub u_AmbientLightColor: i32,
    // pub u_AmbientLightIntensity: i32,

    // // // TODO!: set when integrating IBL (unused now)
    // pub u_DiffuseEnvSampler: i32,
    // pub u_SpecularEnvSampler: i32,
    // pub u_brdfLUT: i32,

    // // ///

    // pub u_BaseColorSampler: i32,
    // pub u_BaseColorFactor: i32,

    // pub u_NormalSampler: i32,
    // pub u_NormalScale: i32,

    // pub u_EmissiveSampler: i32,
    // pub u_EmissiveFactor: i32,

    // pub u_MetallicRoughnessSampler: i32,
    // pub u_MetallicRoughnessValues: i32,

    // pub u_OcclusionSampler: i32,
    // pub u_OcclusionStrength: i32,

    // // // TODO!: use/remove debugging uniforms
    // // // debugging flags used for shader output of intermediate PBR variables
    // pub u_ScaleDiffBaseMR: i32,
    // pub u_ScaleFGDSpec: i32,
    // pub u_ScaleIBLAmbient: i32,
}



impl PbrShader {
    pub fn new(
        gl: Arc<GL>,
        flags: ShaderFlags,
    )
    -> Self 
    {
        // doing some


        let vert_code = include_str!("../shaders/pbr-vert.glsl");
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);
        let vert_shader_log = gl.get_shader_info_log(&vert_shader);
        log!("pbr-vert.glsl compilation log: ", vert_shader_log);
    
        let frag_code = include_str!("../shaders/pbr-frag.glsl");
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);
        let frag_shader_log = gl.get_shader_info_log(&frag_shader);
        log!("pbr-frag.glsl compilation log: ", frag_shader_log);
    
        let shader = Arc::new(gl.create_program().unwrap());
        gl.attach_shader(&shader, &vert_shader);
        gl.attach_shader(&shader, &frag_shader);
        gl.link_program(&shader);
    
    
        
        
        // let mut shader = Shader::from_source(
        //     include_str!("shaders/pbr-vert.glsl"),
        //     include_str!("shaders/pbr-frag.glsl"),
        //     &flags.as_strings());

        // // NOTE: shader debug version
        // // let mut shader = Shader::new(
        // //     "src/shaders/pbr-vert.glsl",
        // //     "src/shaders/pbr-frag.glsl",
        // //     &flags.as_strings());


        // Let's see what we need to reconstruct:
        // We need 

        

        let uniform_locations = unsafe {
            let uniform_locations = PbrUniformLocations {
                u_MVPMatrix: gl.get_uniform_location(&shader, "u_MVPMatrix").unwrap(),
                // u_ModelMatrix: gl.get_uniform_location(&shader, "u_ModelMatrix") as i32,
                // u_Camera: gl.get_attrib_location(&shader, "u_Camera") as i32,
                // // u_MVPMatrix: shader.uniform_location("u_MVPMatrix"),
                // // u_ModelMatrix: shader.uniform_location("u_ModelMatrix"),
                // // u_Camera: shader.uniform_location("u_Camera"),

                // u_LightDirection: gl.get_attrib_location(&shader, "u_LightDirection") as i32,
                // u_LightColor: gl.get_attrib_location(&shader, "u_LightColor") as i32,
                // // u_LightDirection: shader.uniform_location("u_LightDirection"),
                // // u_LightColor: shader.uniform_location("u_LightColor"),


                // u_AmbientLightColor: gl.get_attrib_location(&shader, "u_AmbientLightColor") as i32,

                // u_AmbientLightIntensity: gl.get_attrib_location(&shader, " u_AmbientLightIntensity") as i32,
                // // u_AmbientLightColor: shader.uniform_location("u_AmbientLightColor"),
                // // u_AmbientLightIntensity: shader.uniform_location("u_AmbientLightIntensity"),


                // u_DiffuseEnvSampler: gl.get_attrib_location(&shader, "u_DiffuseEnvSampler") as i32,
                // u_SpecularEnvSampler: gl.get_attrib_location(&shader, "u_SpecularEnvSampler") as i32,
                // u_brdfLUT: gl.get_attrib_location(&shader, "u_brdfLUT") as i32,
                // // u_DiffuseEnvSampler: shader.uniform_location("u_DiffuseEnvSampler"),
                // // u_SpecularEnvSampler: shader.uniform_location("u_SpecularEnvSampler"),
                // // u_brdfLUT: shader.uniform_location("u_brdfLUT"),


                // u_BaseColorSampler: gl.get_attrib_location(&shader, "u_BaseColorSampler") as i32,
                // u_BaseColorFactor: gl.get_attrib_location(&shader, "u_BaseColorFactor") as i32,
                // // u_BaseColorSampler: shader.uniform_location("u_BaseColorSampler"),
                // // u_BaseColorFactor: shader.uniform_location("u_BaseColorFactor"),

                // u_NormalSampler: gl.get_attrib_location(&shader, "u_NormalSampler") as i32,
                // u_NormalScale: gl.get_attrib_location(&shader, "u_NormalScale") as i32,
                // // u_NormalSampler: shader.uniform_location("u_NormalSampler"),
                // // u_NormalScale: shader.uniform_location("u_NormalScale"),

                // u_EmissiveSampler: gl.get_attrib_location(&shader, "u_EmissiveSampler") as i32,
                // u_EmissiveFactor: gl.get_attrib_location(&shader, " u_EmissiveFactor") as i32,
                // // u_EmissiveSampler: shader.uniform_location("u_EmissiveSampler"),
                // // u_EmissiveFactor: shader.uniform_location("u_EmissiveFactor"),

                // u_MetallicRoughnessSampler: gl.get_attrib_location(&shader, "u_MetallicRoughnessSampler") as i32,
                // u_MetallicRoughnessValues: gl.get_attrib_location(&shader, "u_MetallicRoughnessValues") as i32,
                // // u_MetallicRoughnessSampler: shader.uniform_location("u_MetallicRoughnessSampler"),
                // // u_MetallicRoughnessValues: shader.uniform_location("u_MetallicRoughnessValues"),


                // u_OcclusionSampler: gl.get_attrib_location(&shader, "u_OcclusionSampler") as i32,
                // u_OcclusionStrength: gl.get_attrib_location(&shader, "u_OcclusionStrength") as i32,
                // // u_OcclusionSampler: shader.uniform_location("u_OcclusionSampler"),
                // // u_OcclusionStrength: shader.uniform_location("u_OcclusionStrength"),

                // u_ScaleDiffBaseMR: gl.get_attrib_location(&shader, "u_ScaleDiffBaseMR") as i32,
                // u_ScaleFGDSpec: gl.get_attrib_location(&shader, " u_ScaleFGDSpec") as i32,
                // u_ScaleIBLAmbient: gl.get_attrib_location(&shader, "u_ScaleIBLAmbient") as i32,
                // u_ScaleDiffBaseMR: shader.uniform_location("u_ScaleDiffBaseMR"),
                // u_ScaleFGDSpec: shader.uniform_location("u_ScaleFGDSpec"),
                // u_ScaleIBLAmbient: shader.uniform_location("u_ScaleIBLAmbient"),
            };

            // shader.use_program();
            // shader.set_int(uniforms.u_BaseColorSampler, 0);
            // shader.set_int(uniforms.u_NormalSampler, 1);
            // shader.set_int(uniforms.u_EmissiveSampler, 2);
            // shader.set_int(uniforms.u_MetallicRoughnessSampler, 3);
            // shader.set_int(uniforms.u_OcclusionSampler, 4);

            // shader.set_vec3(uniforms.u_LightColor, 5.0, 5.0, 5.0);
            // // TODO!: optional minus on z
            // shader.set_vec3(uniforms.u_LightDirection, 0.0, 0.5, 0.5);

            // shader.set_vec3(uniforms.u_AmbientLightColor, 1.0, 1.0, 1.0);
            // shader.set_float(uniforms.u_AmbientLightIntensity, 0.2);

            uniform_locations
        };

        Self {
            shader,
            flags,
            uniform_locations,
        }
    }
}
