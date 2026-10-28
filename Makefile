CARGO ?= cargo
HARNESS = harness/target/release/clj-torture
JOBS ?= 8
CLOJURE_RS_BIN ?=
FUZZ_COUNT ?= 200

.PHONY: check check-all fuzz fuzz-aot harness jars list clean test

harness:
	$(CARGO) build --release --locked --manifest-path harness/Cargo.toml

jars:
	./scripts/fetch-clojure-jars.sh

test:
	$(CARGO) test --locked --manifest-path harness/Cargo.toml

check: harness jars
	$(HARNESS) run --backend jvm --jobs $(JOBS)

check-all: harness jars
ifdef CLOJURE_RS_BIN
	$(HARNESS) run --backend all --diff-backends --jobs $(JOBS) --clojure-rs-bin $(CLOJURE_RS_BIN)
else
	$(HARNESS) run --backend jvm --jobs $(JOBS)
endif

fuzz: harness jars
	$(HARNESS) fuzz --mode interp --count $(FUZZ_COUNT) --jobs $(JOBS) --clojure-rs-bin $(CLOJURE_RS_BIN)

fuzz-aot: harness jars
	$(HARNESS) fuzz --mode aot --count $(FUZZ_COUNT) --jobs $(JOBS) --clojure-rs-bin $(CLOJURE_RS_BIN)

list: harness
	$(HARNESS) run --list

clean:
	rm -f results.sum
	rm -rf fuzz-out /tmp/torture-aot-* /tmp/torture-read-*.clj
