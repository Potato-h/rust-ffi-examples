ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,--gc-sections -lpthread -ldl
endif

all: target/math
	target/math

target:
	mkdir -p $@

target/math: target/main.o target/debug/libmath.a
	$(CC) -o $@ $^ $(LDFLAGS)

target/debug/libmath.a: src/lib.rs Cargo.toml
	cargo build

target/main.o: src/main.c | target
	$(CC) -o $@ -c $<

clean:
	rm -rf target