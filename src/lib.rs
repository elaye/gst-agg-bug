#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;
extern crate gstreamer_base as gst_base;
extern crate gstreamer_sys as gst_sys;
extern crate gstreamer_video as gst_video;

mod agg;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    agg::register(plugin)?;
    Ok(())
}

gst_plugin_define!(
    aggbug,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "Proprietary",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
