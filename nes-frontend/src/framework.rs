// This entire file is based on https://github.com/parasyte/pixels/tree/main/examples/minimal-egui

use egui::{ClippedPrimitive, Context, TexturesDelta, ViewportId};
use egui_wgpu::{Renderer, RendererOptions, ScreenDescriptor};
use egui_winit::State;
use pixels::{wgpu, PixelsContext};
use tokio::sync::mpsc::Sender;
use winit::{event::WindowEvent, window::Window};

use crate::gui::{Gui, GuiEvent};

pub struct Framework {
    pub gui: Gui,

    egui_ctx: Context,
    egui_state: State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,
}

impl Framework {
    /// `zoom_factor` scales the whole GUI on top of the window's native scale
    /// factor (e.g. `1.2` makes everything 20% bigger).
    pub fn new(
        window: &Window,
        width: u32,
        height: u32,
        zoom_factor: f32,
        pixels: &pixels::Pixels,
        sender: Sender<GuiEvent>,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        egui_ctx.set_zoom_factor(zoom_factor);
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            Some(max_texture_size),
        );
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: egui_winit::pixels_per_point(&egui_ctx, window),
        };
        let renderer = Renderer::new(
            pixels.device(),
            pixels.render_texture_format(),
            RendererOptions::default(),
        );
        let textures = TexturesDelta::default();
        let gui = Gui::new(sender);

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.egui_state.on_window_event(window, event);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    pub fn prepare(&mut self, window: &Window) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run_ui(raw_input, |ui| {
            // Draw the application.
            self.gui.ui(ui.ctx());
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, output.platform_output);
        // `pixels_per_point` is the window's scale factor times egui's zoom
        // factor, so it tracks DPI changes and zoom changes automatically.
        self.screen_descriptor.pixels_per_point = output.pixels_per_point;
        self.paint_jobs = self
            .egui_ctx
            .tessellate(output.shapes, output.pixels_per_point);
    }

    pub fn render(
        &mut self,
        encoder: &mut egui_wgpu::wgpu::CommandEncoder,
        render_target: &egui_wgpu::wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            })
                .forget_lifetime();

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
