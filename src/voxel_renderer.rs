
use graphics::*;
use renderer::*;
use graphics_util::*;
use vector::*;
use math::*;
use std::collections::HashMap;



struct RenderDataProvider{
    pre_render_state: Option<fn()>
}

struct RenderInfo<'a>{
    renderer: &'a RendererVertFrag,
    provider: RenderDataProvider,
}

struct VoxelRenderer<'a>{
    lifetime_one_draw_renderers: HashMap<usize, RenderInfo<'a>>
}
