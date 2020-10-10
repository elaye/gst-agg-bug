export PWD = $(shell pwd)
export LD_LIBRARY_PATH = $(shell echo $(PWD)/target/debug:$(shell rustc --print sysroot)/lib)
export GST_PLUGIN_PATH = $(shell echo $(PWD)/target/debug)

run:
	cargo build
	gst-launch-1.0 \
		videotestsrc ! \
		aggbug ! \
		autovideosink
