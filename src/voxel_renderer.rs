
use graphics::*;
use renderer::*;
use graphics_util::*;
use vector::*;
use math::*;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};

pub enum RenderLifetime{
    Manual,
    OneDraw,
}

pub enum RenderTransform{
    UI,
    World,
    None,
}

#[derive(Clone, Copy)]
pub enum RenderID{
    ID(usize),
}

impl PartialEq for RenderID{
    fn eq(&self, other: &RenderID) -> bool {
        match self{
            &RenderID::ID(x) => {
                match other{
                    &RenderID::ID(y) => x == y
                }
            }
        }
    }
}
impl Eq for RenderID{}

impl Hash for RenderID{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self{
            &RenderID::ID(x) => {
                state.write_usize(x);
                state.finish();
            }
        }
    }
}

struct RenderDataProvider{
    pre_render_state: Option<fn()->()>,
    post_render_state: Option<fn()->()>,
    shader_data: Option<Box<FnOnce(&Program, WindowInfo)->()>>,
}

struct RenderInfo<'a>{
    renderer: &'a RendererVertFrag,
    provider: RenderDataProvider,
}

struct VoxelRenderer<'a>{
    lifetime_one_draw_renderers: HashMap<RenderID, RenderInfo<'a>>,
    lifetime_manual_renderers: HashMap<RenderID, RenderInfo<'a>>,

    shaders: &'a HashMap<String, Program>,
    render_id_counter: usize,
}

impl<'a> VoxelRenderer<'a>{
    fn new(shaders: &'a HashMap<String, Program>) -> VoxelRenderer<'a>{
        VoxelRenderer{
            lifetime_one_draw_renderers: HashMap::new(),
            lifetime_manual_renderers: HashMap::new(),
            shaders,
            render_id_counter: 0,
        }
    }

    fn push(&self, life: RenderLifetime, trans: RenderTransform, mut renderer: RenderInfo<'a>) -> Result<RenderID, &'static str>{
        if !self.shaders.contains_key(&renderer.renderer.shader_name()) {return Err("error: shader not found")}



        fn provide_def(shader: &Program, win: WindowInfo){ //shader will be already enabled
            shader.set_float4x4("P",  false, &[
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0]);
            shader.set_float4x4("V",  false, &[
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0]);
        }

        fn provide_ui(shader: &Program, win: WindowInfo){ //shader will be already enabled
            shader.set_float4x4("P",  false, &ortho(
                                                   0.0,
                                                   win.width as f32,
                                                   win.height as f32,
                                                   0.0,
                                                   -1.0,
                                                   1.0));
            shader.set_float4x4("V",  false, &[
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0]);
        }




        match trans{
            RenderTransform::None => {
                match renderer.provider.shader_data{
                    None => renderer.provider.shader_data = Some(Box::new(provide_def)),
                    Some(x) =>{
                        let provide_def_combined = |shader: &Program, win: WindowInfo|{
                            provide_def(shader, win);
                            x(shader, win);
                        };

                        renderer.provider.shader_data = Some(Box::new(provide_def_combined))
                    }
                }
            },
            RenderTransform::UI => {
              match renderer.provider.shader_data{
                  None => renderer.provider.shader_data = Some(Box::new(provide_ui)),
                  Some(x) => {
                      let provide_ui_combined = |shader: &Program, win: WindowInfo|{
                          provide_ui(shader, win);
                          x(shader, win);
                      };

                      renderer.provider.shader_data = Some(Box::new(provide_ui_combined))
                  }
              }
            },
            RenderTransform::World => {
                unimplemented!();
            }
        };

        let rid = RenderID::ID(self.render_id_counter);


        match life{
            RenderLifetime::Manual => {
                self.lifetime_manual_renderers.insert(rid, renderer);
            },
            RenderLifetime::OneDraw => {
                self.lifetime_one_draw_renderers.insert(rid, renderer);
            }

        };

        Ok(rid)
    }
}