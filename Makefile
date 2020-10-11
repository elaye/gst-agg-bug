export PWD = $(shell pwd)
export LD_LIBRARY_PATH = $(shell echo $(PWD)/target/debug:$(shell rustc --print sysroot)/lib)
export GST_PLUGIN_PATH = $(shell echo $(PWD)/target/debug)
#export G_SLICE=always-malloc
#export G_SLICE=debug-blocks
#export G_SLICE=all
#export G_DEBUG=all
#export MALLOC_CHECK_=3

run:
	cargo build
	#gdb --args gst-launch-1.0
	# valgrind --leak-check=no gst-launch-1.0
	gst-launch-1.0  \
		videotestsrc ! \
		aggbug ! \
		autovideosink
