
use graphics::*;
use renderer::*;
use graphics_util::*;
use vector::*;
use math::*;
use std::collections::HashMap;


struct RenderDataProvider{

}

struct RenderInfo<D: RenderVertFragData>{
    renderer: RendererVertFrag<D>,
    provider: RenderDataProvider,
}

struct VoxelRenderer{
    lifetime_one_draw_renderers: HashMap<usize, >
}