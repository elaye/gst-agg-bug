use glib::{self, subclass, Cast};

use gst::subclass::prelude::*;
use gst::{self, ElementExt, PadExt};
use gst_base::subclass::prelude::*;
use gst_base::{self, prelude::StaticType, AggregatorExt, AggregatorPadExt};

struct Agg {
    sinkpad: gst_base::AggregatorPad,
}

impl ObjectImpl for Agg {
    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);

        let agg = obj.downcast_ref::<gst_base::Aggregator>().unwrap();

        agg.add_pad(&self.sinkpad).unwrap();
    }
}

impl ElementImpl for Agg {}

impl AggregatorImpl for Agg {
    fn aggregate(
        &self,
        agg: &gst_base::Aggregator,
        _timeout: bool,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let current_src_caps = agg.get_static_pad("src").unwrap().get_current_caps();
        let active_caps = self.sinkpad.get_current_caps().unwrap();

        if Some(&active_caps) != current_src_caps.as_ref() {
            agg.set_src_caps(&active_caps);
        }

        let buffer = self.sinkpad.pop_buffer().unwrap();
        self.finish_buffer(agg, buffer)
    }
}

impl ObjectSubclass for Agg {
    const NAME: &'static str = "Agg";
    type ParentType = gst_base::Aggregator;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn with_class(klass: &subclass::simple::ClassStruct<Self>) -> Self {
        let templ = klass.get_pad_template("sink").unwrap();

        let sinkpad: gst_base::AggregatorPad = glib::Object::new(
            gst_base::AggregatorPad::static_type(),
            &[
                ("name", &"sink"),
                ("direction", &gst::PadDirection::Sink),
                ("template", &templ),
            ],
        )
        .unwrap()
        .downcast()
        .unwrap();

        Self { sinkpad }
    }

    fn class_init(klass: &mut subclass::simple::ClassStruct<Self>) {
        klass.set_metadata(
            "AggBug",
            "Generic",
            "Aggregator bug reproduction",
            "Elie Génard <elie.genard@laposte.net>",
        );

        let caps = gst::Caps::new_any();

        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &caps,
        )
        .unwrap();
        klass.add_pad_template(src_pad_template);

        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        )
        .unwrap();
        klass.add_pad_template(sink_pad_template);
    }
}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(Some(plugin), "aggbug", gst::Rank::None, Agg::get_type())
}
