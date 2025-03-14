use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader};

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> WebGlShader {
    let shader = context
        .create_shader(shader_type)
        .expect("Unable to create shader object");
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        shader
    } else {
        panic!(
            "{}",
            context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> WebGlProgram {
    let program = context
        .create_program()
        .expect("Unable to create shader object");

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        program
    } else {
        panic!(
            "{}",
            context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}

/// Safe wrapper around `js_sys` view
pub fn buffer_f32_slice(webgl: &WebGl2RenderingContext, buffer: &WebGlBuffer, data: &[f32]) {
    webgl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer));

    let positions_array_buf_view = unsafe { js_sys::Float32Array::view(data) };

    webgl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &positions_array_buf_view,
        WebGl2RenderingContext::DYNAMIC_DRAW, // Flexible choice but possibly not the most optimal
    );
}

/// Safe wrapper around `js_sys` view
pub fn buffer_u16_indexes(webgl: &WebGl2RenderingContext, buffer: &WebGlBuffer, data: &[u16]) {
    webgl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(buffer));

    let positions_array_buf_view = unsafe { js_sys::Uint16Array::view(data) };

    webgl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        &positions_array_buf_view,
        WebGl2RenderingContext::DYNAMIC_DRAW, // Flexible choice but possibly not the most optimal
    );
}
