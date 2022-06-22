use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGLProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;
    let vert_shader = compile_shader(&gl,Gl::VERTEX_SHADER, vert_source).unwrap();
    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        frag_shader,
    )
}


fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGLShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shadere, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or_else(false)
    {
        Ok(Shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}
